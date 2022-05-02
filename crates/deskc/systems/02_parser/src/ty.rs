use ast::{
    expr::Expr,
    span::Spanned,
    ty::{CommentPosition, Effect, EffectExpr, Type},
};
use chumsky::prelude::*;
use tokens::Token;

use crate::common::{parse_attr, parse_collection, parse_comment, parse_ident};

use super::common::{parse_function, parse_op, ParserExt};

pub fn effect_parser(
    parser: impl Parser<Token, Spanned<Type>, Error = Simple<Token>> + Clone,
) -> impl Parser<Token, Effect, Error = Simple<Token>> + Clone {
    parser
        .clone()
        .then_ignore(just(Token::EArrow))
        .then(parser)
        .map(|(input, output)| Effect { input, output })
}

pub fn parser(
    // Needs this for parse attributes
    expr: impl Parser<Token, Spanned<Expr>, Error = Simple<Token>> + Clone + 'static,
) -> impl Parser<Token, Spanned<Type>, Error = Simple<Token>> + Clone {
    recursive(|type_| {
        let infer = just(Token::Infer).to(Type::Infer);
        let this = just(Token::This).to(Type::This);
        let number = just(Token::NumberType).to(Type::Number);
        let string = just(Token::StringType).to(Type::String);
        let trait_ = just(Token::Trait).ignore_then(
            type_
                .clone()
                .separated_by_comma_at_least_one()
                .map(Type::Trait),
        );
        let alias = just(Token::A).ignore_then(parse_ident().map(Type::Alias));
        let effectful = just(Token::Perform)
            .ignore_then(type_.clone())
            .then(parse_effects(type_.clone()))
            .map(|(ty, effects)| Type::Effectful {
                ty: Box::new(ty),
                effects,
            });
        let product = parse_op(just(Token::Product), type_.clone()).map(Type::Product);
        let sum = parse_op(just(Token::Sum), type_.clone()).map(Type::Sum);
        let array = type_
            .clone()
            .delimited_by(just(Token::ArrayBegin), just(Token::ArrayEnd))
            .map(Box::new)
            .map(Type::Array);
        let set = type_
            .clone()
            .delimited_by(just(Token::SetBegin), just(Token::SetEnd))
            .map(Box::new)
            .map(Type::Set);
        let function = parse_function(
            just(Token::Lambda),
            type_.clone(),
            just(Token::Arrow),
            type_.clone(),
        )
        .map(|(parameters, body)| Type::Function {
            parameters,
            body: Box::new(body),
        });
        let attribute = parse_attr(expr, type_.clone()).map(|(attr, ty)| Type::Attribute {
            attr: Box::new(attr),
            ty: Box::new(ty),
        });
        let brand = filter_map(|span, input| {
            if let Token::Brand(ident) = input {
                Ok(ident)
            } else {
                Err(Simple::custom(span, "Expected brand"))
            }
        })
        .then(type_.clone())
        .map(|(brand, ty)| Type::Brand {
            brand,
            item: Box::new(ty),
        });
        let variable = parse_ident().map(Type::Variable);
        let bound = parse_ident()
            .then_ignore(just(Token::TypeAnnotation))
            .then(type_.clone())
            .map(|(identifier, bound)| Type::BoundedVariable {
                bound: Box::new(bound),
                identifier,
            });
        let let_in = just(Token::Let)
            .ignore_then(parse_ident())
            .in_()
            .then(type_.clone())
            .map(|(definition, expression)| Type::Let {
                variable: definition,
                body: Box::new(expression),
            });

        let prefix_comment =
            parse_comment()
                .then(type_.clone())
                .map(|(text, item)| Type::Comment {
                    position: CommentPosition::Prefix,
                    text,
                    item: Box::new(item),
                });

        infer
            .or(prefix_comment)
            .or(this)
            .or(number)
            .or(string)
            .or(trait_)
            .or(alias)
            .or(effectful)
            .or(product)
            .or(sum)
            .or(array)
            .or(set)
            .or(function)
            .or(brand)
            .or(attribute)
            .or(bound)
            .or(variable)
            .or(let_in)
            .map_with_span(|t, span| (t, span))
            .then(parse_comment().or_not())
            .map_with_span(|(ty, comment), span| {
                if let Some(comment) = comment {
                    (
                        Type::Comment {
                            position: CommentPosition::Suffix,
                            text: comment,
                            item: Box::new(ty),
                        },
                        span,
                    )
                } else {
                    ty
                }
            })
    })
}

fn parse_effects(
    type_: impl Parser<Token, Spanned<Type>, Error = Simple<Token>> + Clone + 'static,
) -> impl Parser<Token, Spanned<EffectExpr>, Error = Simple<Token>> + Clone {
    recursive(|expr| {
        let effects = parse_collection(
            Token::SetBegin,
            type_
                .clone()
                .then_ignore(just(Token::EArrow))
                .then(type_.clone())
                .map(|(input, output)| Effect { input, output })
                .map_with_span(|expr, span| (expr, span)),
            Token::SetEnd,
        )
        .map(EffectExpr::Effects);
        let apply = just(Token::Apply)
            .ignore_then(type_.clone())
            .in_()
            .then(type_.clone().separated_by_comma())
            .map(|(func, arguments)| EffectExpr::Apply {
                function: Box::new(func),
                arguments,
            });

        let add = parse_op(just(Token::Sum), expr.clone()).map(EffectExpr::Add);
        let sub = just(Token::Minus)
            .ignore_then(expr.clone())
            .then_ignore(just(Token::Comma))
            .then(expr.clone())
            .map(|(minuend, subtrahend)| EffectExpr::Sub {
                minuend: Box::new(minuend),
                subtrahend: Box::new(subtrahend),
            });
        effects
            .labelled("effects")
            .or(apply.labelled("effects apply"))
            .or(add.labelled("effects add"))
            .or(sub.labelled("effects sub"))
            .map_with_span(|expr, span| (expr, span))
    })
}

#[cfg(test)]
mod tests {
    use ast::expr::{Expr, Literal};
    use chumsky::Stream;
    use lexer::lexer;

    use crate::expr;

    use super::*;

    fn parse(input: &str) -> Result<Spanned<Type>, Vec<Simple<Token>>> {
        parser(expr::parser())
            .then_ignore(end())
            .parse(Stream::from_iter(
                input.len()..input.len() + 1,
                lexer().then_ignore(end()).parse(input).unwrap().into_iter(),
            ))
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse("'number").unwrap().0, Type::Number);
    }

    #[test]
    fn parse_trait() {
        let trait_ = dbg!(parse("% 'number, _.").unwrap().0);

        if let Type::Trait(trait_) = trait_ {
            assert_eq!(trait_.len(), 2);
            assert_eq!(trait_[0].0, Type::Number);
            assert_eq!(trait_[1].0, Type::Infer);
        } else {
            panic!("Expected trait");
        }
    }

    #[test]
    fn parse_type_alias() {
        assert_eq!(
            parse("'a something").unwrap().0,
            Type::Alias("something".into())
        );
    }

    #[test]
    fn parse_single_token() {
        assert_eq!(parse("_").unwrap().0, Type::Infer);
        assert_eq!(parse("'this").unwrap().0, Type::This);
    }

    #[test]
    fn parse_product_and_sum() {
        assert_eq!(
            parse("* + 'number, _., *").unwrap().0,
            Type::Product(vec![
                (
                    Type::Sum(vec![(Type::Number, 4..11), (Type::Infer, 13..14)]),
                    2..15
                ),
                (Type::Product(vec![]), 17..18)
            ])
        );
    }

    #[test]
    fn parse_function() {
        assert_eq!(
            parse(r#"\ 'number, 'number -> _"#).unwrap().0,
            Type::Function {
                parameters: vec![(Type::Number, 2..9), (Type::Number, 11..18)],
                body: Box::new((Type::Infer, 22..23)),
            }
        );
    }

    #[test]
    fn parse_array() {
        assert_eq!(
            parse("['number]").unwrap().0,
            Type::Array(Box::new((Type::Number, 1..8)))
        );
    }

    #[test]
    fn parse_set() {
        assert_eq!(
            parse("{'number}").unwrap().0,
            Type::Set(Box::new((Type::Number, 1..8),))
        );
    }

    #[test]
    fn parse_bound() {
        assert_eq!(
            parse("a: 'a bound").unwrap().0,
            Type::BoundedVariable {
                identifier: "a".into(),
                bound: Box::new((Type::Alias("bound".into()), 3..11)),
            }
        );
    }

    #[test]
    fn parse_effectful() {
        assert_eq!(
            parse("! result + {'number => 'string}, - >a _, {'string => 'number}")
                .unwrap()
                .0,
            Type::Effectful {
                ty: Box::new((Type::Variable("result".into()), 2..8)),
                effects: (
                    EffectExpr::Add(vec![
                        (
                            EffectExpr::Effects(vec![(
                                Effect {
                                    input: (Type::Number, 12..19),
                                    output: (Type::String, 23..30),
                                },
                                12..30
                            )]),
                            11..31
                        ),
                        (
                            EffectExpr::Sub {
                                minuend: Box::new((
                                    EffectExpr::Apply {
                                        function: Box::new((Type::Variable("a".into()), 36..37)),
                                        arguments: vec![(Type::Infer, 38..39)],
                                    },
                                    35..39
                                )),
                                subtrahend: Box::new((
                                    EffectExpr::Effects(vec![(
                                        Effect {
                                            input: (Type::String, 42..49),
                                            output: (Type::Number, 53..60),
                                        },
                                        42..60
                                    ),],),
                                    41..61
                                ))
                            },
                            33..61
                        ),
                    ]),
                    9..61
                ),
            }
        );
    }

    #[test]
    fn parse_brand() {
        assert_eq!(
            parse("@added 'number").unwrap().0,
            Type::Brand {
                brand: "added".into(),
                item: Box::new((Type::Number, 7..14)),
            }
        );
    }

    #[test]
    fn parse_attribute() {
        assert_eq!(
            parse("#1 ~ 'number").unwrap().0,
            Type::Attribute {
                attr: Box::new((Expr::Literal(Literal::Int(1)), 1..2)),
                ty: Box::new((Type::Number, 5..12)),
            }
        );
    }

    #[test]
    fn parse_let() {
        assert_eq!(
            parse("$ x ~ x").unwrap().0,
            Type::Let {
                variable: "x".into(),
                body: Box::new((Type::Variable("x".into()), 6..7))
            }
        );
    }

    #[test]
    fn parse_comment() {
        assert_eq!(
            parse("(a)*(b)").unwrap().0,
            Type::Comment {
                position: CommentPosition::Prefix,
                text: "(a)".into(),
                item: Box::new((
                    Type::Comment {
                        position: CommentPosition::Suffix,
                        text: "(b)".into(),
                        item: Box::new((Type::Product(vec![]), 3..4)),
                    },
                    3..7
                )),
            }
        );
    }
}
