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
        ty::{Effect, EffectExpr, Function, Trait, Type},
    };
    use dson::Dson;
    use ids::{LinkName, NodeId};

    use super::*;

    fn w<T>(value: T) -> WithSpan<T> {
        WithSpan {
            id: NodeId::new(),
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

    #[test]
    fn ident_with_spaces() {
        assert_eq!(
            parse("'type the\t\nnumber  of apples 'number; ?")
                .unwrap()
                .value,
            Expr::NewType {
                ident: "the number of apples".into(),
                ty: w(Type::Number),
                expr: bw(Expr::Hole),
            }
        );
    }

    #[test]
    fn ident_utf8() {
        assert_eq!(
            parse("'type あ-　a0 'number; ?").unwrap().value,
            Expr::NewType {
                ident: "あ- a0".into(),
                ty: w(Type::Number),
                expr: bw(Expr::Hole),
            }
        );
    }

    #[test]
    fn string_with_escape() {
        assert_eq!(
            parse(
                r#"
            "\\\n\""
            "#
            )
            .unwrap()
            .value,
            Expr::Literal(Literal::String("\\\n\"".into()))
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
            .unwrap()
            .value,
            Expr::Literal(Literal::String("the\t\nnumber  of apples".into()))
        );
    }

    #[test]
    fn parse_literal_int() {
        assert_eq!(
            parse("-12").unwrap().value,
            Expr::Literal(Literal::Integer(-12))
        );
    }

    #[test]
    fn parse_literal_rational() {
        assert_eq!(
            parse("1/2").unwrap().value,
            Expr::Literal(Literal::Rational(1, 2))
        );
    }

    #[test]
    fn parse_literal_string() {
        assert_eq!(
            parse(r#""abc""#).unwrap().value,
            Expr::Literal(Literal::String("abc".into()))
        );
    }

    #[test]
    fn parse_let() {
        assert_eq!(
            parse("$ 3; ?").unwrap().value,
            Expr::Let {
                definition: bw(Expr::Literal(Literal::Integer(3))),
                body: bw(Expr::Hole),
            }
        );
    }

    #[test]
    fn parse_perform() {
        assert_eq!(
            parse("! ? ~> 'string").unwrap().value,
            Expr::Perform {
                input: bw(Expr::Hole),
                output: Some(w(Type::String)),
            }
        );
    }

    #[test]
    fn parse_handle() {
        let trait_ = parse(r#"'handle ? { 'number ~> 'string => 3 }"#)
            .unwrap()
            .value;
        assert_eq!(
            trait_,
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
        assert_eq!(
            parse("^add(1 2)").unwrap().value,
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
            parse("& x").unwrap().value,
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
            parse("*<1, ?>").unwrap().value,
            Expr::Product(vec![w(Expr::Literal(Literal::Integer(1))), w(Expr::Hole),])
        );
    }

    #[test]
    fn parse_function() {
        assert_eq!(
            parse(r#"\ 'number -> ?"#).unwrap().value,
            Expr::Function {
                parameter: w(Type::Number),
                body: bw(Expr::Hole),
            },
        );
    }

    #[test]
    fn parse_array() {
        assert_eq!(
            parse("[1, ?, ?]").unwrap().value,
            Expr::Vector(vec![
                w(Expr::Literal(Literal::Integer(1))),
                w(Expr::Hole),
                w(Expr::Hole),
            ])
        );
    }

    #[test]
    fn parse_set() {
        assert_eq!(
            parse("{1 => 1, 2 => ?, ? => 3}").unwrap().value,
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
            parse(": 'number ?").unwrap().value,
            Expr::Typed {
                item: bw(Expr::Hole),
                ty: w(Type::Number),
            }
        );
    }

    #[test]
    fn parse_attribute() {
        assert_eq!(
            parse("# 3 ?").unwrap().value,
            Expr::Attributed {
                attr: Dson::Literal(dson::Literal::Integer(3)),
                item: bw(Expr::Hole),
            }
        );
    }

    #[test]
    fn parse_brand() {
        assert_eq!(
            parse(r#"'brand "a"; ?"#).unwrap().value,
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
              'string => "string".
            }
            "#
            )
            .unwrap()
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
            parse(r#"@"true" *"#).unwrap().value,
            Expr::Label {
                label: Dson::Literal(dson::Literal::String("true".into())),
                item: bw(Expr::Product(vec![])),
            }
        );
    }

    #[test]
    fn parse_comment() {
        assert_eq!(
            parse("(a)*").unwrap().value,
            Expr::Comment {
                text: "(a)".into(),
                item: bw(Expr::Product(vec![])),
            }
        );
    }

    #[test]
    fn test_type() {
        assert_eq!(parse("&'number").unwrap().value, r(Type::Number));
    }

    #[test]
    fn parse_trait() {
        let trait_ = parse("& %<\'number -> 'number>").unwrap().value;

        if let Expr::Apply {
            function:
                WithSpan {
                    value: Type::Trait(trait_),
                    ..
                },
            ..
        } = trait_
        {
            assert_eq!(
                trait_,
                Trait(vec![w(Function {
                    parameter: w(Type::Number),
                    body: w(Type::Number),
                })])
            );
        } else {
            panic!("Expected trait");
        }
    }

    #[test]
    fn parse_variable() {
        assert_eq!(
            parse("& 'a something").unwrap().value,
            r(Type::Variable("something".into()))
        );
    }

    #[test]
    fn parse_single_token() {
        assert_eq!(parse("& _").unwrap().value, r(Type::Infer));
        assert_eq!(parse("& 'this").unwrap().value, r(Type::This));
    }

    #[test]
    fn parse_product_and_sum() {
        assert_eq!(
            parse("& *< +<'number, _> *<> >").unwrap().value,
            r(Type::Product(vec![
                w(Type::Sum(vec![w(Type::Number), w(Type::Infer)])),
                w(Type::Product(vec![]))
            ]))
        );
    }

    #[test]
    fn parse_function_ty() {
        assert_eq!(
            parse(r#"& \ 'number -> _"#).unwrap().value,
            r(Type::Function(Box::new(Function {
                parameter: w(Type::Number),
                body: w(Type::Infer),
            })))
        );
    }

    #[test]
    fn parse_vec_ty() {
        assert_eq!(
            parse("& ['number]").unwrap().value,
            r(Type::Vector(bw(Type::Number)))
        );
    }

    #[test]
    fn parse_map_ty() {
        assert_eq!(
            parse("{'number => 'string}").unwrap().value,
            r(Type::Map {
                key: bw(Type::Number),
                value: bw(Type::String),
            })
        );
    }

    #[test]
    fn parse_bound() {
        assert_eq!(
            parse("& 'forall a: %<> a").unwrap().value,
            r(Type::Forall {
                variable: "a".into(),
                bound: Some(bw(Type::Trait(Trait(vec![])))),
                body: bw(Type::Variable("a".into())),
            })
        );
    }

    #[test]
    fn parse_effectful() {
        assert_eq!(
            parse("& ! result + {'number => 'string}, - >a _, {'string => 'number}")
                .unwrap()
                .value,
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
            parse("& @added 'number").unwrap().value,
            r(Type::Brand {
                brand: Dson::Literal(dson::Literal::String("added".into())),
                item: bw(Type::Number),
            })
        );
    }

    #[test]
    fn parse_attribute_ty() {
        assert_eq!(
            parse("& #1 'number").unwrap().value,
            r(Type::Attributed {
                attr: Dson::Literal(dson::Literal::Integer(1)),
                ty: bw(Type::Number),
            })
        );
    }

    #[test]
    fn parse_let_ty() {
        assert_eq!(
            parse("& $ x a; x").unwrap().value,
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
            parse("& (a)*<>").unwrap().value,
            r(Type::Comment {
                text: "(a)".into(),
                item: bw(Type::Product(vec![])),
            })
        );
    }
}
