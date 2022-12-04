pub mod grammar_trait;
pub mod parser;

mod conversions;
mod grammar;

use ast::{expr::Expr, span::WithSpan};
use thiserror::Error;

pub use parol_runtime::derive_builder;

pub fn parse(input: &str) -> Result<WithSpan<Expr>, MinimalistSyntaxError> {
    let mut grammar = grammar::Grammar::new();
    parser::parse(input, "dummy", &mut grammar)
        .map_err(|err| MinimalistSyntaxError::ParseError(err.to_string()))?;
    grammar.expr.unwrap().try_into()
}

#[derive(Error, Debug)]
pub enum MinimalistSyntaxError {
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Uuid error: {0}")]
    UuidError(#[from] uuid::Error),
    #[error("Dson error: {0}")]
    DsonError(#[from] ast::dson::ExprToDsonError),
}

#[cfg(test)]
mod tests {
    use ast::{
        expr::{Handler, Literal, MapElem, MatchCase},
        remove_span::remove_span,
        ty::{Effect, EffectExpr, Function, Type},
    };
    use dson::Dson;
    use ids::{LinkName, NodeId};

    use super::*;

    fn w<T>(value: T) -> WithSpan<T> {
        WithSpan {
            id: NodeId::default(),
            span: 0..0,
            value,
        }
    }

    fn bw<T>(value: T) -> Box<WithSpan<T>> {
        Box::new(w(value))
    }

    fn r(value: Type) -> Expr {
        Expr::Apply {
            function: w(value),
            link_name: LinkName::None,
            arguments: vec![],
        }
    }

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn parse(input: &str) -> WithSpan<Expr> {
        let mut expr = super::parse(input).unwrap();
        remove_span(&mut expr);
        expr
    }

    #[test]
    fn ident_with_spaces() {
        assert_eq!(
            parse("& `the\t\nnumber  of apples`").value,
            r(Type::Variable("the number of apples".into()))
        );
    }

    #[test]
    #[ignore = "parol #42"]
    fn ident_utf8() {
        init();
        assert_eq!(
            parse("& `あ-　a 0  `").value,
            r(Type::Variable("あ- a 0".into()))
        );
    }

    #[test]
    fn string_with_escape() {
        assert_eq!(
            parse(
                r#"
            "\\\n\"\t"
            "#
            )
            .value,
            Expr::Literal(Literal::String("\\\n\"\t".into()))
        );
    }

    #[test]
    fn string_with_spaces() {
        assert_eq!(
            parse(
                r#"
            "the\t\nnumber  of apples"
            "#
            )
            .value,
            Expr::Literal(Literal::String("the\t\nnumber  of apples".into()))
        );
    }

    #[test]
    fn parse_literal_int() {
        assert_eq!(parse("-12").value, Expr::Literal(Literal::Integer(-12)));
        assert_eq!(parse("0x11").value, Expr::Literal(Literal::Integer(17)));
        assert_eq!(parse("0o11").value, Expr::Literal(Literal::Integer(9)));
        assert_eq!(parse("0b11").value, Expr::Literal(Literal::Integer(3)));
    }

    #[test]
    fn parse_literal_rational() {
        assert_eq!(
            parse("12 / 23").value,
            Expr::Literal(Literal::Rational(12, 23))
        );
    }

    #[test]
    fn parse_literal_string() {
        assert_eq!(
            parse(r#""abc""#).value,
            Expr::Literal(Literal::String("abc".into()))
        );
    }

    #[test]
    fn parse_let() {
        assert_eq!(
            parse("$ 3; ?").value,
            Expr::Let {
                definition: bw(Expr::Literal(Literal::Integer(3))),
                body: bw(Expr::Hole),
            }
        );
    }

    #[test]
    fn parse_perform() {
        assert_eq!(
            parse("! ? ~> 'string").value,
            Expr::Perform {
                input: bw(Expr::Hole),
                output: w(Type::String),
            }
        );
    }

    #[test]
    fn parse_handle() {
        assert_eq!(
            parse(r#"'handle ? { 'number ~> 'string => 3 }"#).value,
            Expr::Handle {
                expr: bw(Expr::Hole),
                handlers: vec![Handler {
                    input: w(Type::Number),
                    output: w(Type::String),
                    handler: w(Expr::Literal(Literal::Integer(3))),
                }],
            }
        );
    }

    #[test]
    fn parse_call() {
        init();
        assert_eq!(
            parse("^add(1 2)").value,
            Expr::Apply {
                function: w(Type::Variable("add".into())),
                link_name: LinkName::None,
                arguments: vec![
                    w(Expr::Literal(Literal::Integer(1))),
                    w(Expr::Literal(Literal::Integer(2)))
                ],
            }
        );
    }

    #[test]
    fn parse_reference() {
        assert_eq!(
            parse("& x").value,
            Expr::Apply {
                function: w(Type::Variable("x".into())),
                link_name: LinkName::None,
                arguments: vec![],
            }
        );
    }

    #[test]
    fn parse_product() {
        assert_eq!(
            parse("*<1, ?>").value,
            Expr::Product(vec![w(Expr::Literal(Literal::Integer(1))), w(Expr::Hole),])
        );
    }

    #[test]
    fn parse_function() {
        assert_eq!(
            parse(r#"\ 'number -> ?"#).value,
            Expr::Function {
                parameter: w(Type::Number),
                body: bw(Expr::Hole),
            },
        );
    }

    #[test]
    fn parse_array() {
        assert_eq!(
            parse("[1, ?, ?]").value,
            Expr::Vector(vec![
                w(Expr::Literal(Literal::Integer(1))),
                w(Expr::Hole),
                w(Expr::Hole),
            ])
        );
    }

    #[test]
    fn parse_map() {
        assert_eq!(
            parse("{1 => 1, 2 => ?, ? => 3}").value,
            Expr::Map(vec![
                MapElem {
                    key: w(Expr::Literal(Literal::Integer(1))),
                    value: w(Expr::Literal(Literal::Integer(1))),
                },
                MapElem {
                    key: w(Expr::Literal(Literal::Integer(2))),
                    value: w(Expr::Hole)
                },
                MapElem {
                    key: w(Expr::Hole),
                    value: w(Expr::Literal(Literal::Integer(3)))
                },
            ])
        );
    }

    #[test]
    fn parse_type_annotation() {
        assert_eq!(
            parse(": 'number ?").value,
            Expr::Typed {
                item: bw(Expr::Hole),
                ty: w(Type::Number),
            }
        );
    }

    #[test]
    fn parse_attribute() {
        assert_eq!(
            parse("# 3 ?").value,
            Expr::Attributed {
                attr: Dson::Literal(dson::Literal::Integer(3)),
                item: bw(Expr::Hole),
            }
        );
    }

    #[test]
    fn parse_brand() {
        assert_eq!(
            parse(r#"'brand "a"; ?"#).value,
            Expr::Brand {
                brand: Dson::Literal(dson::Literal::String("a".into())),
                item: bw(Expr::Hole),
            }
        );
    }

    #[test]
    fn parse_match() {
        assert_eq!(
            parse(
                r#"
            'match ? {
              'number => "number",
              'string => "string",
            }
            "#
            )
            .value,
            Expr::Match {
                of: bw(Expr::Hole),
                cases: vec![
                    MatchCase {
                        ty: w(Type::Number),
                        expr: w(Expr::Literal(Literal::String("number".into()))),
                    },
                    MatchCase {
                        ty: w(Type::String),
                        expr: w(Expr::Literal(Literal::String("string".into()))),
                    },
                ]
            }
        );
    }

    #[test]
    fn parse_label() {
        assert_eq!(
            parse(r#"@"true" *<>"#).value,
            Expr::Label {
                label: Dson::Literal(dson::Literal::String("true".into())),
                item: bw(Expr::Product(vec![])),
            }
        );
    }

    #[test]
    fn parse_comment() {
        assert_eq!(
            parse("~(a))~*<>").value,
            Expr::Comment {
                text: "a)".into(),
                item: bw(Expr::Product(vec![])),
            }
        );
    }

    #[test]
    fn test_type() {
        assert_eq!(parse("& 'number").value, r(Type::Number));
    }

    #[test]
    fn parse_trait() {
        assert_eq!(
            parse(r#"& %<\'number -> 'number>"#).value,
            r(Type::Trait(vec![w(Function {
                parameter: w(Type::Number),
                body: w(Type::Number),
            })]))
        );
    }

    #[test]
    fn parse_variable() {
        assert_eq!(
            parse("& something").value,
            r(Type::Variable("something".into()))
        );
    }

    #[test]
    fn parse_single_token() {
        assert_eq!(parse("& _").value, r(Type::Infer));
        assert_eq!(parse("& 'this").value, r(Type::This));
    }

    #[test]
    fn parse_product_and_sum() {
        assert_eq!(
            parse("& *< +<'number, _> *<> >").value,
            r(Type::Product(vec![
                w(Type::Sum(vec![w(Type::Number), w(Type::Infer)])),
                w(Type::Product(vec![]))
            ]))
        );
    }

    #[test]
    fn parse_function_ty() {
        assert_eq!(
            parse(r#"& \ 'number -> _"#).value,
            r(Type::Function(Box::new(Function {
                parameter: w(Type::Number),
                body: w(Type::Infer),
            })))
        );
    }

    #[test]
    fn parse_vec_ty() {
        assert_eq!(
            parse("& ['number]").value,
            r(Type::Vector(bw(Type::Number)))
        );
    }

    #[test]
    fn parse_map_ty() {
        assert_eq!(
            parse("& {'number => 'string}").value,
            r(Type::Map {
                key: bw(Type::Number),
                value: bw(Type::String),
            })
        );
    }

    #[test]
    fn parse_bound() {
        assert_eq!(
            parse("& 'forall a: %<> a").value,
            r(Type::Forall {
                variable: "a".into(),
                bound: Some(bw(Type::Trait(vec![]))),
                body: bw(Type::Variable("a".into())),
            })
        );
    }

    #[test]
    fn parse_effectful() {
        assert_eq!(
            parse("& ! +< {'number ~> 'string}, - < ^a(_), {'string ~> 'number} >> result").value,
            r(Type::Effectful {
                ty: bw(Type::Variable("result".into())),
                effects: w(EffectExpr::Add(vec![
                    w(EffectExpr::Effects(vec![w(Effect {
                        input: w(Type::Number),
                        output: w(Type::String),
                    },)]),),
                    w(EffectExpr::Sub {
                        minuend: bw(EffectExpr::Apply {
                            function: bw(Type::Variable("a".into())),
                            arguments: vec![w(Type::Infer)],
                        },),
                        subtrahend: bw(EffectExpr::Effects(vec![w(Effect {
                            input: w(Type::String),
                            output: w(Type::Number),
                        },)]))
                    })
                ]))
            })
        );
    }

    #[test]
    fn parse_brand_ty() {
        assert_eq!(
            parse(r#"& @"added" 'number"#).value,
            r(Type::Brand {
                brand: Dson::Literal(dson::Literal::String("added".into())),
                item: bw(Type::Number),
            })
        );
    }

    #[test]
    fn parse_attribute_ty() {
        assert_eq!(
            parse("& #1 'number").value,
            r(Type::Attributed {
                attr: Dson::Literal(dson::Literal::Integer(1)),
                ty: bw(Type::Number),
            })
        );
    }

    #[test]
    fn parse_let_ty() {
        assert_eq!(
            parse("& $ x a; x").value,
            r(Type::Let {
                variable: "x".into(),
                definition: bw(Type::Variable("a".into())),
                body: bw(Type::Variable("x".into())),
            })
        );
    }

    #[test]
    fn parse_comment_ty() {
        assert_eq!(
            parse("& ~(a)~*<>").value,
            r(Type::Comment {
                text: "a".into(),
                item: bw(Type::Product(vec![])),
            })
        );
    }
}
