use ast::{
    span::Spanned,
    ty::{Effect, Type},
};
use chumsky::prelude::*;
use tokens::Token;

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
                type_
                    .clone()
                    .separated_by(just(Token::Comma))
                    .at_least(1)
                    .map(|types| Type::Trait(types)),
            )
            .dot();
        let alias = just(Token::A).ignore_then(filter_map(|span, token| match token {
            Token::Ident(ident) => Ok(Type::Alias(ident)),
            _ => Err(Simple::custom(span, "Expected identifier")),
        }));
        let effectful = just(Token::Perform)
            .ignore_then(type_.clone())
            .in_()
            .then(
                type_
                    .clone()
                    .then_ignore(just(Token::EArrow))
                    .then(type_.clone())
                    .map(|(input, output)| Effect { input, output })
                    .separated_by(just(Token::Comma))
                    .at_least(1)
                    .dot(),
            )
            .map(|(ty, effects)| Type::Effectful {
                ty: Box::new(ty),
                effects,
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
        let variable = identifier.clone().map(Type::Variable);

        infer
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
    fn parse_effectful() {
        assert_eq!(
            parse("! 'a result ~ 'number => 'string, 'string => 'number.")
                .unwrap()
                .0,
            Type::Effectful {
                ty: Box::new((Type::Alias("result".into()), 2..11)),
                effects: vec![
                    Effect {
                        input: (Type::Number, 14..21),
                        output: (Type::String, 25..32),
                    },
                    Effect {
                        input: (Type::String, 34..41),
                        output: (Type::Number, 45..52),
                    }
                ]
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
}
