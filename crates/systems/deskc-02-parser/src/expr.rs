use ast::{
    expr::{Expr, Handler, LinkName, Literal, MatchCase},
    span::WithSpan,
    ty::{CommentPosition, Type},
};
use chumsky::prelude::*;
use ids::NodeId;
use tokens::Token;

use crate::common::{parse_attr, parse_comment, parse_ident, parse_uuid};

use super::common::{parse_collection, parse_function, parse_op, parse_typed, ParserExt};

pub fn parser() -> impl Parser<Token, WithSpan<Expr>, Error = Simple<Token>> + Clone {
    recursive(|expr| {
        let hole = just(Token::Hole).to(Expr::Hole);
        let int64 = filter_map(|span, token| match token {
            Token::Int(int) => Ok(int),
            _ => Err(Simple::custom(span, "expected int literal")),
        });
        let rational = int64
            .then_ignore(just(Token::Divide))
            .then(int64)
            .map(|(a, b)| Expr::Literal(Literal::Rational(a, b)));
        let string = filter_map(|span, token| match token {
            Token::Str(string) => Ok(Expr::Literal(Literal::String(string))),
            _ => Err(Simple::custom(span, "expected string literal")),
        });
        let literal = rational
            .or(int64.map(|int| Expr::Literal(Literal::Integer(int))))
            .or(string);
        let type_ = super::ty::parser(expr.clone());
        let let_in = just(Token::Let)
            .ignore_then(expr.clone())
            // TODO: span for Type::Infer
            .then(
                just(Token::TypeAnnotation)
                    .ignore_then(type_.clone())
                    .or_not()
                    .map(|ty| {
                        ty.unwrap_or(WithSpan {
                            id: NodeId::new(),
                            value: Type::Infer,
                            span: 0..0,
                        })
                    }),
            )
            .in_()
            .then(expr.clone())
            .map(|((definition, ty), expression)| Expr::Let {
                ty,
                definition: Box::new(definition),
                body: Box::new(expression),
            });
        let perform = just(Token::Perform)
            .ignore_then(expr.clone())
            .then_ignore(just(Token::EArrow))
            .then(type_.clone())
            .map(|(effect, output)| Expr::Perform {
                input: Box::new(effect),
                output,
            });
        let continue_ = just(Token::Continue)
            .ignore_then(expr.clone())
            .then(just(Token::EArrow).ignore_then(type_.clone()).or_not())
            .map(|(expr, output)| Expr::Continue {
                input: Box::new(expr),
                output,
            });
        let handle = just(Token::Handle)
            .ignore_then(expr.clone())
            .in_()
            .then(
                type_
                    .clone()
                    .then_ignore(just(Token::EArrow))
                    .then(type_.clone())
                    .then_ignore(just(Token::Arrow).or_not())
                    .then(expr.clone())
                    .map(|((input, output), handler)| Handler {
                        input,
                        output,
                        handler,
                    })
                    .separated_by_comma_at_least_one(),
            )
            .map(|(expr, handlers)| Expr::Handle {
                expr: Box::new(expr),
                handlers,
            });
        let card_uuid = just(Token::Card)
            .ignore_then(parse_uuid())
            .map(LinkName::Card)
            .or_not()
            .map(|name| name.unwrap_or(LinkName::None));
        let apply = just(Token::Apply)
            .ignore_then(type_.clone())
            .then(card_uuid.clone())
            .in_()
            .then(expr.clone().separated_by_comma_at_least_one())
            .map(|((function, link_name), arguments)| Expr::Apply {
                function,
                link_name,
                arguments,
            });
        let reference = just(Token::Reference)
            .ignore_then(type_.clone())
            .then(card_uuid)
            .map(|(reference, link_name)| Expr::Apply {
                function: reference,
                link_name,
                arguments: vec![],
            });
        let product = parse_op(just(Token::Product), expr.clone()).map(Expr::Product);
        let function = parse_function(
            just(Token::Lambda),
            type_.clone(),
            just(Token::Arrow),
            expr.clone(),
        )
        .map(|(parameters, body)| Expr::Function {
            parameters,
            body: Box::new(body),
        });
        let array =
            parse_collection(Token::ArrayBegin, expr.clone(), Token::ArrayEnd).map(Expr::Vector);
        let set = parse_collection(Token::SetBegin, expr.clone(), Token::SetEnd).map(Expr::Set);
        let typed = parse_typed(expr.clone(), type_.clone()).map(|(expr, ty)| Expr::Typed {
            ty,
            item: Box::new(expr),
        });
        let attribute =
            parse_attr(expr.clone(), expr.clone()).map(|(attr, expr)| Expr::Attribute {
                attr: Box::new(attr),
                item: Box::new(expr),
            });
        let brand = just(Token::Brands)
            .ignore_then(parse_ident().separated_by_comma())
            .in_()
            .then(expr.clone())
            .map(|(brands, expr)| Expr::Brand {
                brands,
                item: Box::new(expr),
            });
        let match_ = just(Token::Sum)
            .ignore_then(expr.clone())
            .in_()
            .then(
                type_
                    .clone()
                    .then_ignore(just(Token::Arrow))
                    .then(expr.clone())
                    .map(|(ty, expr)| MatchCase { ty, expr })
                    .separated_by_comma_at_least_one(),
            )
            .map(|(of, cases)| Expr::Match {
                of: Box::new(of),
                cases,
            });
        let label = filter_map(|span, input| {
            if let Token::Brand(ident) = input {
                Ok(ident)
            } else {
                Err(Simple::custom(span, "Expected brand"))
            }
        })
        .then(expr.clone())
        .map(|(label, expr)| Expr::Label {
            label,
            item: Box::new(expr),
        });
        let newtype = just(Token::Type)
            .ignore_then(parse_ident())
            .then(type_.clone())
            .in_()
            .then(expr.clone())
            .map(|((ident, ty), expr)| Expr::NewType {
                ident,
                ty,
                expr: Box::new(expr),
            });
        let prefix_comment = parse_comment()
            .then(expr.clone())
            .map(|(text, expr)| Expr::Comment {
                position: CommentPosition::Prefix,
                text,
                item: Box::new(expr),
            });
        let card = just(Token::Card)
            .ignore_then(parse_uuid())
            .then(expr.clone())
            .in_()
            .then(expr.clone().or_not())
            .map(|((uuid, item), next)| Expr::Card {
                uuid,
                item: Box::new(item),
                next: next.map(Box::new),
            });

        hole.or(prefix_comment)
            .or(literal.labelled("literal"))
            .or(let_in.labelled("let-in"))
            .or(perform.labelled("perform"))
            .or(continue_.labelled("continue"))
            .or(product.labelled("product"))
            .or(array.labelled("array"))
            .or(set.labelled("set"))
            .or(typed.labelled("typed"))
            .or(attribute.labelled("attribute"))
            .or(brand.labelled("brand"))
            .or(match_.labelled("match"))
            .or(function.labelled("function"))
            .or(apply.labelled("apply"))
            .or(reference.labelled("reference"))
            .or(handle.labelled("handle"))
            .or(label.labelled("label"))
            .or(newtype.labelled("newtype"))
            .or(card.labelled("card"))
            .map_with_span(|token, span| WithSpan {
                id: NodeId::new(),
                value: token,
                span,
            })
            .then(parse_comment().or_not())
            .map_with_span(|(expr, comment), span| {
                if let Some(comment) = comment {
                    WithSpan {
                        id: NodeId::new(),
                        value: Expr::Comment {
                            position: CommentPosition::Suffix,
                            text: comment,
                            item: Box::new(expr),
                        },
                        span,
                    }
                } else {
                    expr
                }
            })
    })
}

#[cfg(test)]
mod tests {
    use ast::{expr::MatchCase, remove_span::remove_span, span::dummy_span, ty::Type};
    use lexer::scan;

    use crate::ParserError;

    use super::*;

    fn parse(input: &str) -> Result<WithSpan<Expr>, ParserError> {
        crate::parse(scan(input).unwrap()).map(|mut with_span| {
            remove_span(&mut with_span);
            with_span
        })
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
            parse("$ 3: 'number ~ ?").unwrap().value,
            Expr::Let {
                ty: dummy_span(Type::Number),
                definition: Box::new(dummy_span(Expr::Literal(Literal::Integer(3)))),
                body: Box::new(dummy_span(Expr::Hole)),
            }
        );
    }

    #[test]
    fn parse_perform() {
        assert_eq!(
            parse("! ? => 'string").unwrap().value,
            Expr::Perform {
                input: Box::new(dummy_span(Expr::Hole)),
                output: dummy_span(Type::String),
            }
        );
    }

    #[test]
    fn parse_handle() {
        let trait_ = parse(r#"'handle ? ~ 'number => 'string -> 3"#)
            .unwrap()
            .value;
        assert_eq!(
            trait_,
            Expr::Handle {
                expr: Box::new(dummy_span(Expr::Hole)),
                handlers: vec![Handler {
                    input: dummy_span(Type::Number),
                    output: dummy_span(Type::String),
                    handler: dummy_span(Expr::Literal(Literal::Integer(3))),
                }],
            }
        );
    }

    #[test]
    fn parse_call() {
        assert_eq!(
            parse("> 'a add ~ 1, 2.").unwrap().value,
            Expr::Apply {
                function: dummy_span(Type::Variable("add".into())),
                link_name: LinkName::None,
                arguments: vec![
                    dummy_span(Expr::Literal(Literal::Integer(1))),
                    dummy_span(Expr::Literal(Literal::Integer(2)))
                ],
            }
        );
    }

    #[test]
    fn parse_reference() {
        assert_eq!(
            parse("& 'a x").unwrap().value,
            Expr::Apply {
                function: dummy_span(Type::Variable("x".into())),
                link_name: LinkName::None,
                arguments: vec![],
            }
        );
    }

    #[test]
    fn parse_product() {
        assert_eq!(
            parse("* 1, ?").unwrap().value,
            Expr::Product(vec![
                dummy_span(Expr::Literal(Literal::Integer(1))),
                dummy_span(Expr::Hole),
            ])
        );
    }

    #[test]
    fn parse_function() {
        assert_eq!(
            parse(r#"\ 'number, _ -> ?"#).unwrap().value,
            Expr::Function {
                parameters: vec![dummy_span(Type::Number), dummy_span(Type::Infer)],
                body: Box::new(dummy_span(Expr::Hole)),
            },
        );
    }

    #[test]
    fn parse_array() {
        assert_eq!(
            parse("[1, ?, ?]").unwrap().value,
            Expr::Vector(vec![
                dummy_span(Expr::Literal(Literal::Integer(1))),
                dummy_span(Expr::Hole),
                dummy_span(Expr::Hole),
            ])
        );
    }

    #[test]
    fn parse_set() {
        assert_eq!(
            parse("{1, ?, ?}").unwrap().value,
            Expr::Set(vec![
                dummy_span(Expr::Literal(Literal::Integer(1))),
                dummy_span(Expr::Hole),
                dummy_span(Expr::Hole),
            ])
        );
    }

    #[test]
    fn parse_type_annotation() {
        assert_eq!(
            parse("^?: 'number").unwrap().value,
            Expr::Typed {
                item: Box::new(dummy_span(Expr::Hole)),
                ty: dummy_span(Type::Number),
            }
        );
    }

    #[test]
    fn parse_attribute() {
        assert_eq!(
            parse("# 3 ~ ?").unwrap().value,
            Expr::Attribute {
                attr: Box::new(dummy_span(Expr::Literal(Literal::Integer(3)))),
                item: Box::new(dummy_span(Expr::Hole)),
            }
        );
    }

    #[test]
    fn parse_brand() {
        assert_eq!(
            parse("'brand a, b. ~ ?").unwrap().value,
            Expr::Brand {
                brands: vec!["a".into(), "b".into()],
                item: Box::new(dummy_span(Expr::Hole)),
            }
        );
    }

    #[test]
    fn parse_match() {
        assert_eq!(
            parse(
                r#"
            + ? ~
            'number -> "number",
            'string -> "string".
            "#
            )
            .unwrap()
            .value,
            Expr::Match {
                of: Box::new(dummy_span(Expr::Hole)),
                cases: vec![
                    MatchCase {
                        ty: dummy_span(Type::Number),
                        expr: dummy_span(Expr::Literal(Literal::String("number".into()))),
                    },
                    MatchCase {
                        ty: dummy_span(Type::String),
                        expr: dummy_span(Expr::Literal(Literal::String("string".into()))),
                    },
                ]
            }
        );
    }

    #[test]
    fn parse_match_without_in() {
        assert_eq!(
            parse(
                r#"
            + & 'a x
            'number -> "number",
            'string -> "string".
            "#
            )
            .unwrap()
            .value,
            Expr::Match {
                of: Box::new(dummy_span(Expr::Apply {
                    function: dummy_span(Type::Variable("x".into())),
                    link_name: LinkName::None,
                    arguments: vec![]
                },)),
                cases: vec![
                    MatchCase {
                        ty: dummy_span(Type::Number),
                        expr: dummy_span(Expr::Literal(Literal::String("number".into()))),
                    },
                    MatchCase {
                        ty: dummy_span(Type::String),
                        expr: dummy_span(Expr::Literal(Literal::String("string".into()))),
                    },
                ]
            }
        );
    }

    #[test]
    fn parse_label() {
        assert_eq!(
            parse("@true *").unwrap().value,
            Expr::Label {
                label: "true".into(),
                item: Box::new(dummy_span(Expr::Product(vec![]))),
            }
        );
    }

    #[test]
    fn parse_comment() {
        assert_eq!(
            parse("(a)*(b)").unwrap().value,
            Expr::Comment {
                position: CommentPosition::Prefix,
                text: "(a)".into(),
                item: Box::new(dummy_span(Expr::Comment {
                    position: CommentPosition::Suffix,
                    text: "(b)".into(),
                    item: Box::new(dummy_span(Expr::Product(vec![]))),
                },)),
            }
        );
    }
}
