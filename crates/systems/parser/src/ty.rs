use ast::{
    span::Spanned,
    ty::{Handler, Type},
};
use chumsky::prelude::*;
use tokens::Token;

use super::common::{parse_effectful, parse_function, parse_op, ParserExt};

pub fn effect_parser(
    parser: impl Parser<Token, Spanned<Type>, Error = Simple<Token>> + Clone,
) -> impl Parser<Token, Handler, Error = Simple<Token>> + Clone {
    parser
        .clone()
        .then_ignore(just(Token::EArrow))
        .then(parser)
        .map(|(input, output)| Handler { input, output })
}

pub fn parser() -> impl Parser<Token, Spanned<Type>, Error = Simple<Token>> + Clone {
    let identifier = filter_map(|span, token| {
        if let Token::Ident(ident) = token {
            Ok(ident)
        } else {
            Err(Simple::custom(span, "Expected identifier"))
        }
    });
    let type_ = recursive(|type_| {
        let infer = just(Token::Infer).to(Type::Infer);
        let this = just(Token::This).to(Type::This);
        let number = just(Token::NumberType).to(Type::Number);
        let string = just(Token::StringType).to(Type::String);
        let trait_ = just(Token::Trait)
            .ignore_then(
                effect_parser(type_.clone())
                    .separated_by(just(Token::Comma))
                    .at_least(1)
                    .map(|types| Type::Class(types))
                    .or(type_
                        .clone()
                        .separated_by(just(Token::Comma))
                        .at_least(1)
                        .map(|types| Type::Trait(types))),
            )
            .dot();
        let alias = just(Token::A).ignore_then(filter_map(|span, token| match token {
            Token::Ident(ident) => Ok(Type::Alias(ident)),
            _ => Err(Simple::custom(span, "Expected identifier")),
        }));
        let effectful =
            parse_effectful(type_.clone(), type_.clone()).map(|(class, ty, handlers)| {
                Type::Effectful {
                    class: Box::new(class),
                    ty: Box::new(ty),
                    handlers: handlers
                        .into_iter()
                        .map(|handler| Handler {
                            input: handler.0,
                            output: handler.1,
                        })
                        .collect(),
                }
            });
        let product =
            parse_op(just(Token::Product), type_.clone()).map(|types| Type::Product(types));
        let sum = parse_op(just(Token::Sum), type_.clone()).map(|types| Type::Sum(types));
        let array = type_
            .clone()
            .delimited_by(Token::ArrayBegin, Token::ArrayEnd)
            .map(Box::new)
            .map(Type::Array);
        let set = type_
            .clone()
            .delimited_by(Token::SetBegin, Token::SetEnd)
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
        let effect = just(Token::Perform)
            .ignore_then(type_.clone())
            .in_()
            .then(effect_parser(type_.clone()))
            .map(|(class, handler)| Type::Effect {
                class: Box::new(class),
                handler: Box::new(handler),
            });
        let variable = identifier.clone().map(Type::Variable);

        infer
            .or(this)
            .or(number)
            .or(string)
            .or(trait_)
            .or(alias)
            .or(effectful)
            .or(effect)
            .or(product)
            .or(sum)
            .or(array)
            .or(set)
            .or(function)
            .or(variable.clone())
            .map_with_span(|t, span| (t, span))
    });

    let bound = identifier
        .then_ignore(just(Token::TypeAnnotation))
        .then(type_.clone())
        .map_with_span(|(identifier, bound), span| {
            (
                Type::BoundedVariable {
                    bound: Box::new(bound),
                    identifier,
                },
                span,
            )
        });

    bound.or(type_)
}

#[cfg(test)]
mod tests {
    use chumsky::Stream;
    use lexer::lexer;

    use super::*;

    fn parse(input: &str) -> Result<Spanned<Type>, Vec<Simple<Token>>> {
        dbg!(parser()
            .then_ignore(end())
            .parse_recovery_verbose(Stream::from_iter(
                input.len()..input.len() + 1,
                dbg!(lexer().then_ignore(end()).parse(input).unwrap().into_iter()),
            )));
        parser().then_ignore(end()).parse(Stream::from_iter(
            input.len()..input.len() + 1,
            dbg!(lexer().then_ignore(end()).parse(input).unwrap().into_iter()),
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
    fn parse_handler_in_class() {
        let trait_ = parse("% 'number => _.").unwrap().0;

        if let Type::Class(class) = trait_ {
            assert_eq!(class.len(), 1);
            assert_eq!(
                class[0],
                Handler {
                    input: (Type::Number, 2..9),
                    output: (Type::Infer, 13..14),
                }
            );
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
        assert_eq!(parse("_").unwrap().0, Type::Infer);
        assert_eq!(parse("&").unwrap().0, Type::This);
    }

    #[test]
    fn parse_effectful() {
        assert_eq!(
            parse("# 'a class ~ 'number ; 'a num_to_num => _, 'a str_to_str => _")
                .unwrap()
                .0,
            Type::Effectful {
                class: Box::new((Type::Alias("class".into()), 2..10)),
                ty: Box::new((Type::Number, 13..20)),
                handlers: vec![
                    Handler {
                        input: (Type::Alias("num_to_num".into()), 23..36),
                        output: (Type::Infer, 40..41),
                    },
                    Handler {
                        input: (Type::Alias("str_to_str".into()), 43..56),
                        output: (Type::Infer, 60..61),
                    },
                ],
            }
        );
    }

    #[test]
    fn parse_product_and_sum() {
        assert_eq!(
            parse("* + 'number, _., *.").unwrap().0,
            Type::Product(vec![
                (
                    Type::Sum(vec![(Type::Number, 4..11), (Type::Infer, 13..14)]),
                    2..15
                ),
                (Type::Product(vec![]), 17..19)
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
    fn parse_effect() {
        assert_eq!(
            parse("! 'a trait ~ 'number => _").unwrap().0,
            Type::Effect {
                class: Box::new((Type::Alias("trait".into()), 2..10)),
                handler: Box::new(Handler {
                    input: (Type::Number, 13..20),
                    output: (Type::Infer, 24..25),
                })
            }
        );
    }
}
