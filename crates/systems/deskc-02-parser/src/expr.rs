use ast::{
    expr::{Expr, Handler, LinkName, Literal, MatchCase},
    span::Spanned,
    ty::{CommentPosition, Type},
};
use chumsky::prelude::*;
use tokens::Token;

use crate::common::{parse_attr, parse_comment, parse_ident, parse_uuid};

use super::common::{parse_collection, parse_function, parse_op, parse_typed, ParserExt};

pub fn parser() -> impl Parser<Token, Spanned<Expr>, Error = Simple<Token>> + Clone {
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
            .or(int64.map(|int| Expr::Literal(Literal::Int(int))))
            .or(string);
        let type_ = super::ty::parser(expr.clone());
        let let_in = just(Token::Let)
            .ignore_then(expr.clone())
            // TODO: span for Type::Infer
            .then(
                just(Token::TypeAnnotation)
                    .ignore_then(type_.clone())
                    .or_not()
                    .map(|ty| ty.unwrap_or((Type::Infer, 0..0))),
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
        let include = just(Token::Include)
            .ignore_then(parse_ident())
            .map(Expr::Include);
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
            .or(include.labelled("include"))
            .or(function.labelled("function"))
            .or(apply.labelled("apply"))
            .or(reference.labelled("reference"))
            .or(handle.labelled("handle"))
            .or(label.labelled("label"))
            .or(newtype.labelled("newtype"))
            .or(card.labelled("card"))
            .map_with_span(|token, span| (token, span))
            .then(parse_comment().or_not())
            .map_with_span(|(expr, comment), span| {
                if let Some(comment) = comment {
                    (
                        Expr::Comment {
                            position: CommentPosition::Suffix,
                            text: comment,
                            item: Box::new(expr),
                        },
                        span,
                    )
                } else {
                    expr
                }
            })
    })
}

#[cfg(test)]
mod tests {
    use ast::{expr::MatchCase, ty::Type};
    use lexer::scan;

    use crate::ParserError;

    use super::*;

    fn parse(input: &str) -> Result<Spanned<Expr>, ParserError> {
        crate::parse(scan(input).unwrap())
    }

    #[test]
    fn parse_literal_int() {
        assert_eq!(parse("-12").unwrap().0, Expr::Literal(Literal::Int(-12)));
    }

    #[test]
    fn parse_literal_rational() {
        assert_eq!(
            parse("1/2").unwrap().0,
            Expr::Literal(Literal::Rational(1, 2))
        );
    }

    #[test]
    fn parse_literal_string() {
        assert_eq!(
            parse(r#""abc""#).unwrap().0,
            Expr::Literal(Literal::String("abc".into()))
        );
    }

    #[test]
    fn parse_let() {
        assert_eq!(
            parse("$ 3: 'number ~ ?").unwrap().0,
            Expr::Let {
                ty: (Type::Number, 5..12),
                definition: Box::new((Expr::Literal(Literal::Int(3)), 2..3)),
                body: Box::new((Expr::Hole, 15..16)),
            }
        );
    }

    #[test]
    fn parse_perform() {
        assert_eq!(
            parse("! ? => 'string").unwrap().0,
            Expr::Perform {
                input: Box::new((Expr::Hole, 2..3)),
                output: (Type::String, 7..14),
            }
        );
    }

    #[test]
    fn parse_handle() {
        let trait_ = parse(r#"'handle ? ~ 'number => 'string -> 3"#).unwrap().0;
        assert_eq!(
            trait_,
            Expr::Handle {
                expr: Box::new((Expr::Hole, 8..9)),
                handlers: vec![Handler {
                    input: (Type::Number, 12..19),
                    output: (Type::String, 23..30),
                    handler: (Expr::Literal(Literal::Int(3)), 34..35),
                }],
            }
        );
    }

    #[test]
    fn parse_call() {
        assert_eq!(
            parse("> 'a add ~ 1, 2.").unwrap().0,
            Expr::Apply {
                function: (Type::Alias("add".into()), 2..8),
                link_name: LinkName::None,
                arguments: vec![
                    (Expr::Literal(Literal::Int(1)), 11..12),
                    (Expr::Literal(Literal::Int(2)), 14..15)
                ],
            }
        );
    }

    #[test]
    fn parse_reference() {
        assert_eq!(
            parse("& 'a x").unwrap().0,
            Expr::Apply {
                function: (Type::Alias("x".into()), 2..6),
                link_name: LinkName::None,
                arguments: vec![],
            }
        );
    }

    #[test]
    fn parse_product() {
        assert_eq!(
            parse("* 1, ?").unwrap().0,
            Expr::Product(vec![
                (Expr::Literal(Literal::Int(1)), 2..3),
                (Expr::Hole, 5..6),
            ])
        );
    }

    #[test]
    fn parse_function() {
        assert_eq!(
            parse(r#"\ 'number, _ -> ?"#).unwrap().0,
            Expr::Function {
                parameters: vec![(Type::Number, 2..9), (Type::Infer, 11..12)],
                body: Box::new((Expr::Hole, 16..17)),
            },
        );
    }

    #[test]
    fn parse_array() {
        assert_eq!(
            parse("[1, ?, ?]").unwrap().0,
            Expr::Vector(vec![
                (Expr::Literal(Literal::Int(1)), 1..2),
                (Expr::Hole, 4..5),
                (Expr::Hole, 7..8),
            ])
        );
    }

    #[test]
    fn parse_set() {
        assert_eq!(
            parse("{1, ?, ?}").unwrap().0,
            Expr::Set(vec![
                (Expr::Literal(Literal::Int(1)), 1..2),
                (Expr::Hole, 4..5),
                (Expr::Hole, 7..8),
            ])
        );
    }

    #[test]
    fn parse_type_annotation() {
        assert_eq!(
            parse("^?: 'number").unwrap().0,
            Expr::Typed {
                item: Box::new((Expr::Hole, 1..2)),
                ty: (Type::Number, 4..11),
            }
        );
    }

    #[test]
    fn parse_attribute() {
        assert_eq!(
            parse("# 3 ~ ?").unwrap().0,
            Expr::Attribute {
                attr: Box::new((Expr::Literal(Literal::Int(3)), 2..3)),
                item: Box::new((Expr::Hole, 6..7)),
            }
        );
    }

    #[test]
    fn parse_brand() {
        assert_eq!(
            parse("'brand a, b. ~ ?").unwrap().0,
            Expr::Brand {
                brands: vec!["a".into(), "b".into()],
                item: Box::new((Expr::Hole, 15..16)),
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
            .0,
            Expr::Match {
                of: Box::new((Expr::Hole, 15..16)),
                cases: vec![
                    MatchCase {
                        ty: (Type::Number, 31..38),
                        expr: (Expr::Literal(Literal::String("number".into())), 42..50),
                    },
                    MatchCase {
                        ty: (Type::String, 64..71),
                        expr: (Expr::Literal(Literal::String("string".into())), 75..83),
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
            .0,
            Expr::Match {
                of: Box::new((
                    Expr::Apply {
                        function: (Type::Alias("x".into()), 17..21),
                        link_name: LinkName::None,
                        arguments: vec![]
                    },
                    15..21
                )),
                cases: vec![
                    MatchCase {
                        ty: (Type::Number, 34..41),
                        expr: (Expr::Literal(Literal::String("number".into())), 45..53),
                    },
                    MatchCase {
                        ty: (Type::String, 67..74),
                        expr: (Expr::Literal(Literal::String("string".into())), 78..86),
                    },
                ]
            }
        );
    }

    #[test]
    fn parse_label() {
        assert_eq!(
            parse("@true *").unwrap().0,
            Expr::Label {
                label: "true".into(),
                item: Box::new((Expr::Product(vec![]), 6..7)),
            }
        );
    }

    #[test]
    fn parse_comment() {
        assert_eq!(
            parse("(a)*(b)").unwrap().0,
            Expr::Comment {
                position: CommentPosition::Prefix,
                text: "(a)".into(),
                item: Box::new((
                    Expr::Comment {
                        position: CommentPosition::Suffix,
                        text: "(b)".into(),
                        item: Box::new((Expr::Product(vec![]), 3..4)),
                    },
                    3..7
                )),
            }
        );
    }
}
