use chumsky::prelude::*;

use crate::{lexer::Token, span::Spanned};

use super::common::parse_effectful;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Number,
    String,
    Trait(Vec<Spanned<Self>>),
    Handler {
        input: Box<Spanned<Self>>,
        output: Box<Spanned<Self>>,
    },
    Effectful {
        class: Box<Spanned<Self>>,
        ty: Box<Spanned<Self>>,
        handlers: Vec<Self>,
    },
    Hole,
    Infer,
    Alias(String),
}

pub fn effect_parser(
    parser: impl Parser<Token, Spanned<Type>, Error = Simple<Token>> + Clone,
) -> impl Parser<Token, Spanned<Type>, Error = Simple<Token>> + Clone {
    parser
        .clone()
        .then_ignore(just(Token::EArrow))
        .then(parser)
        .map(|(input, output)| Type::Handler {
            input: Box::new(input),
            output: Box::new(output),
        })
        .map_with_span(|t, span| (t, span))
}

pub fn parser() -> impl Parser<Token, Spanned<Type>, Error = Simple<Token>> + Clone {
    recursive(|type_| {
        let hole = just(Token::Hole).to(Type::Hole);
        let number = just(Token::NumberType).to(Type::Number);
        let trait_ = just(Token::Trait)
            .ignore_then(
                effect_parser(type_.clone())
                    .or(type_.clone())
                    .separated_by(just(Token::Comma)),
            )
            .map(|types| Type::Trait(types));
        let alias = just(Token::A).ignore_then(filter_map(|span, token| match token {
            Token::Ident(ident) => Ok(Type::Alias(ident)),
            _ => Err(Simple::custom(span, "Expected identifier")),
        }));
        let infer = just(Token::Infer).to(Type::Infer);
        let effectful =
            parse_effectful(type_.clone(), type_.clone()).map(|(class, ty, handlers)| {
                Type::Effectful {
                    class: Box::new(class),
                    ty: Box::new(ty),
                    handlers: handlers
                        .into_iter()
                        .map(|handler| Type::Handler {
                            input: Box::new(handler.0),
                            output: Box::new(handler.1),
                        })
                        .collect(),
                }
            });
        hole.or(number)
            .or(trait_)
            .or(alias)
            .or(infer)
            .or(effectful)
            .then_ignore(just(Token::Dot).or_not())
            .map_with_span(|t, span| (t, span))
    })
}

#[cfg(test)]
mod tests {
    use chumsky::Stream;
    use matches::assert_matches;

    use crate::lexer::lexer;

    use super::*;

    fn parse(input: &str) -> Result<Spanned<Type>, Vec<Simple<Token>>> {
        parser().parse(Stream::from_iter(
            input.len()..input.len() + 1,
            dbg!(lexer().parse(input).unwrap().into_iter()),
        ))
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse("'number").unwrap().0, Type::Number);
    }

    #[test]
    fn ignore_dot() {
        assert_eq!(parse("'number.").unwrap().0, Type::Number);
    }

    #[test]
    fn parse_trait() {
        let trait_ = parse("% 'number, ?").unwrap().0;

        if let Type::Trait(trait_) = trait_ {
            assert_eq!(trait_.len(), 2);
            assert_eq!(trait_[0].0, Type::Number);
            assert_eq!(trait_[1].0, Type::Hole);
        } else {
            panic!("Expected trait");
        }
    }

    #[test]
    fn parse_handler_in_trait() {
        let trait_ = parse("% 'number => ?").unwrap().0;

        if let Type::Trait(trait_) = trait_ {
            assert_eq!(trait_.len(), 1);
            assert_matches!(
                trait_[0].0,
                Type::Handler {
                    input: box (Type::Number, _),
                    output: box (Type::Hole, _),
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
    fn parse_infer_token() {
        assert_eq!(parse("_").unwrap().0, Type::Infer);
    }

    #[test]
    fn parse_effectful() {
        assert_eq!(
            parse("# 'a class 'number ; 'a num_to_num => ?; 'a str_to_str => _")
                .unwrap()
                .0,
            Type::Effectful {
                class: Box::new((Type::Alias("class".into()), 2..10)),
                ty: Box::new((Type::Number, 11..18)),
                handlers: vec![
                    Type::Handler {
                        input: Box::new((Type::Alias("num_to_num".into()), 21..34)),
                        output: Box::new((Type::Hole, 38..39)),
                    },
                    Type::Handler {
                        input: Box::new((Type::Alias("str_to_str".into()), 41..54)),
                        output: Box::new((Type::Infer, 58..59)),
                    },
                ],
            }
        );
    }
}
