use ast::{
    expr::Expr,
    span::WithSpan,
    ty::{CommentPosition, Effect, EffectExpr, Type},
};
use chumsky::prelude::*;
use ids::NodeId;
use tokens::Token;

use crate::common::{parse_attr, parse_collection, parse_comment, parse_ident};

use super::common::{parse_function, parse_op, ParserExt};

pub fn effect_parser(
    parser: impl Parser<Token, WithSpan<Type>, Error = Simple<Token>> + Clone,
) -> impl Parser<Token, Effect, Error = Simple<Token>> + Clone {
    parser
        .clone()
        .then_ignore(just(Token::EArrow))
        .then(parser)
        .map(|(input, output)| Effect { input, output })
}

pub fn parser(
    // Needs this for parse attributes
    expr: impl Parser<Token, WithSpan<Expr>, Error = Simple<Token>> + Clone + 'static,
) -> impl Parser<Token, WithSpan<Type>, Error = Simple<Token>> + Clone {
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
            .map(Type::Vector);
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
        let brand = just(Token::Brand)
            .ignore_then(parse_ident())
            .then(type_.clone())
            .map(|(brand, ty)| Type::Brand {
                brand,
                item: Box::new(ty),
            });
        let variable = just(Token::A)
            .or_not()
            .ignore_then(parse_ident())
            .map(Type::Variable);
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
            .map_with_span(|t, span| WithSpan {
                id: NodeId::new(),
                value: t,
                span,
            })
            .then(parse_comment().or_not())
            .map_with_span(|(ty, comment), span| {
                if let Some(comment) = comment {
                    WithSpan {
                        id: NodeId::new(),
                        value: Type::Comment {
                            position: CommentPosition::Suffix,
                            text: comment,
                            item: Box::new(ty),
                        },
                        span,
                    }
                } else {
                    ty
                }
            })
    })
}

fn parse_effects(
    type_: impl Parser<Token, WithSpan<Type>, Error = Simple<Token>> + Clone + 'static,
) -> impl Parser<Token, WithSpan<EffectExpr>, Error = Simple<Token>> + Clone {
    recursive(|expr| {
        let effects = parse_collection(
            Token::SetBegin,
            type_
                .clone()
                .then_ignore(just(Token::EArrow))
                .then(type_.clone())
                .map(|(input, output)| Effect { input, output })
                .map_with_span(|expr, span| WithSpan {
                    id: NodeId::new(),
                    value: expr,
                    span,
                }),
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
            .map_with_span(|expr, span| WithSpan {
                id: NodeId::new(),
                value: expr,
                span,
            })
    })
}

#[cfg(test)]
mod tests {
    use ast::{
        expr::{Expr, Literal},
        remove_span::remove_span_ty,
        span::dummy_span,
    };
    use chumsky::Stream;
    use lexer::lexer;

    use crate::expr;

    use super::*;

    fn parse(input: &str) -> Result<WithSpan<Type>, Vec<Simple<Token>>> {
        parser(expr::parser())
            .then_ignore(end())
            .parse(Stream::from_iter(
                input.len()..input.len() + 1,
                lexer().then_ignore(end()).parse(input).unwrap().into_iter(),
            ))
            .map(|mut with_span| {
                remove_span_ty(&mut with_span);
                with_span
            })
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse("'number").unwrap().value, Type::Number);
    }

    #[test]
    fn parse_trait() {
        let trait_ = parse("% 'number, _.").unwrap().value;

        if let Type::Trait(trait_) = trait_ {
            assert_eq!(trait_.len(), 2);
            assert_eq!(trait_[0].value, Type::Number);
            assert_eq!(trait_[1].value, Type::Infer);
        } else {
            panic!("Expected trait");
        }
    }

    #[test]
    fn parse_variable() {
        assert_eq!(
            parse("'a something").unwrap().value,
            Type::Variable("something".into())
        );
    }

    #[test]
    fn parse_single_token() {
        assert_eq!(parse("_").unwrap().value, Type::Infer);
        assert_eq!(parse("'this").unwrap().value, Type::This);
    }

    #[test]
    fn parse_product_and_sum() {
        assert_eq!(
            parse("* + 'number, _., *").unwrap().value,
            Type::Product(vec![
                dummy_span(Type::Sum(vec![
                    dummy_span(Type::Number),
                    dummy_span(Type::Infer)
                ])),
                dummy_span(Type::Product(vec![]))
            ])
        );
    }

    #[test]
    fn parse_function() {
        assert_eq!(
            parse(r#"\ 'number, 'number -> _"#).unwrap().value,
            Type::Function {
                parameters: vec![dummy_span(Type::Number), dummy_span(Type::Number)],
                body: Box::new(dummy_span(Type::Infer)),
            }
        );
    }

    #[test]
    fn parse_array() {
        assert_eq!(
            parse("['number]").unwrap().value,
            Type::Vector(Box::new(dummy_span(Type::Number)))
        );
    }

    #[test]
    fn parse_set() {
        assert_eq!(
            parse("{'number}").unwrap().value,
            Type::Set(Box::new(dummy_span(Type::Number),))
        );
    }

    #[test]
    fn parse_bound() {
        assert_eq!(
            parse("a: 'a bound").unwrap().value,
            Type::BoundedVariable {
                identifier: "a".into(),
                bound: Box::new(dummy_span(Type::Variable("bound".into()))),
            }
        );
    }

    #[test]
    fn parse_effectful() {
        assert_eq!(
            parse("! result + {'number => 'string}, - >a _, {'string => 'number}")
                .unwrap()
                .value,
            Type::Effectful {
                ty: Box::new(dummy_span(Type::Variable("result".into()))),
                effects: dummy_span(EffectExpr::Add(vec![
                    dummy_span(EffectExpr::Effects(vec![dummy_span(Effect {
                        input: dummy_span(Type::Number),
                        output: dummy_span(Type::String),
                    },)]),),
                    dummy_span(EffectExpr::Sub {
                        minuend: Box::new(dummy_span(EffectExpr::Apply {
                            function: Box::new(dummy_span(Type::Variable("a".into()))),
                            arguments: vec![dummy_span(Type::Infer)],
                        },)),
                        subtrahend: Box::new(dummy_span(EffectExpr::Effects(vec![dummy_span(
                            Effect {
                                input: dummy_span(Type::String),
                                output: dummy_span(Type::Number),
                            },
                        )])))
                    })
                ]))
            }
        );
    }

    #[test]
    fn parse_brand() {
        assert_eq!(
            parse("@added 'number").unwrap().value,
            Type::Brand {
                brand: "added".into(),
                item: Box::new(dummy_span(Type::Number)),
            }
        );
    }

    #[test]
    fn parse_attribute() {
        assert_eq!(
            parse("#1 ~ 'number").unwrap().value,
            Type::Attribute {
                attr: Box::new(dummy_span(Expr::Literal(Literal::Integer(1)))),
                ty: Box::new(dummy_span(Type::Number)),
            }
        );
    }

    #[test]
    fn parse_let() {
        assert_eq!(
            parse("$ x ~ x").unwrap().value,
            Type::Let {
                variable: "x".into(),
                body: Box::new(dummy_span(Type::Variable("x".into())))
            }
        );
    }

    #[test]
    fn parse_comment() {
        assert_eq!(
            parse("(a)*(b)").unwrap().value,
            Type::Comment {
                position: CommentPosition::Prefix,
                text: "(a)".into(),
                item: Box::new(dummy_span(Type::Comment {
                    position: CommentPosition::Suffix,
                    text: "(b)".into(),
                    item: Box::new(dummy_span(Type::Product(vec![]))),
                },)),
            }
        );
    }
}
