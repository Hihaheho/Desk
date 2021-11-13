use chumsky::prelude::*;

use crate::{lexer::Token, span::Spanned};

use super::common::{parse_collection, parse_effectful, parse_function, parse_op, ParserExt};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Handler {
    input: Spanned<Type>,
    output: Spanned<Type>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Number,
    String,
    Trait(Vec<Spanned<Self>>),
    // Handlers do not need to be spanned because it has not leading token.
    Class(Vec<Handler>),
    Effectful {
        class: Box<Spanned<Self>>,
        ty: Box<Spanned<Self>>,
        handlers: Vec<Handler>,
    },
    Hole,
    Infer,
    This,
    Alias(String),
    Product(Vec<Spanned<Self>>),
    Sum(Vec<Spanned<Self>>),
    Function {
        parameters: Vec<Spanned<Self>>,
        body: Box<Spanned<Self>>,
    },
    Array(Vec<Spanned<Self>>),
    Set(Vec<Spanned<Self>>),
}

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
    recursive(|type_| {
        let hole = just(Token::Hole).to(Type::Hole);
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
        let array =
            parse_collection(Token::ArrayBegin, type_.clone(), Token::ArrayEnd).map(Type::Array);
        let set = parse_collection(Token::SetBegin, type_.clone(), Token::SetEnd).map(Type::Set);
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

        hole.or(infer)
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
        let trait_ = dbg!(parse("% 'number, ?.").unwrap().0);

        if let Type::Trait(trait_) = trait_ {
            assert_eq!(trait_.len(), 2);
            assert_eq!(trait_[0].0, Type::Number);
            assert_eq!(trait_[1].0, Type::Hole);
        } else {
            panic!("Expected trait");
        }
    }

    #[test]
    fn parse_handler_in_class() {
        let trait_ = parse("% 'number => ?.").unwrap().0;

        if let Type::Class(class) = trait_ {
            assert_eq!(class.len(), 1);
            assert_matches!(
                class[0],
                Handler {
                    input: (Type::Number, _),
                    output: (Type::Hole, _),
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
        assert_eq!(parse("?").unwrap().0, Type::Hole);
        assert_eq!(parse("&").unwrap().0, Type::This);
    }

    #[test]
    fn parse_effectful() {
        assert_eq!(
            parse("# 'a class 'number ; 'a num_to_num => ?, 'a str_to_str => _")
                .unwrap()
                .0,
            Type::Effectful {
                class: Box::new((Type::Alias("class".into()), 2..10)),
                ty: Box::new((Type::Number, 11..18)),
                handlers: vec![
                    Handler {
                        input: (Type::Alias("num_to_num".into()), 21..34),
                        output: (Type::Hole, 38..39),
                    },
                    Handler {
                        input: (Type::Alias("str_to_str".into()), 41..54),
                        output: (Type::Infer, 58..59),
                    },
                ],
            }
        );
    }

    #[test]
    fn parse_product_and_sum() {
        assert_eq!(
            parse("* + 'number, ?., *.").unwrap().0,
            Type::Product(vec![
                (
                    Type::Sum(vec![(Type::Number, 4..11), (Type::Hole, 13..14)]),
                    2..15
                ),
                (Type::Product(vec![]), 17..19)
            ])
        );
    }

    #[test]
    fn parse_function() {
        assert_eq!(
            parse(r#"\ 'number, 'number -> ?"#).unwrap().0,
            Type::Function {
                parameters: vec![(Type::Number, 2..9), (Type::Number, 11..18)],
                body: Box::new((Type::Hole, 22..23)),
            }
        );
    }

	#[test]
	fn parse_array() {
		assert_eq!(
			parse("[?, ?, 'number]").unwrap().0,
			Type::Array(vec![
				(Type::Hole, 1..2),
				(Type::Hole, 4..5),
				(Type::Number, 7..14),
			])
		);
	}

	#[test]
	fn parse_set() {
		assert_eq!(
			parse("{?, ?, 'number}").unwrap().0,
			Type::Set(vec![
				(Type::Hole, 1..2),
				(Type::Hole, 4..5),
				(Type::Number, 7..14),
			])
		);
	}
}
