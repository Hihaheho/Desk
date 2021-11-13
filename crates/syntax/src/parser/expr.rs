use chumsky::prelude::*;
use uuid::Uuid;

use crate::{lexer::Token, span::Spanned};

use super::{common::{ParserExt, parse_collection, parse_effectful, parse_function, parse_op}, r#type::{self, Type}};

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    String(String),
    Int(i64),
    Rational(i64, i64),
    Float(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Handler {
    pub ty: Spanned<Type>,
    pub expr: Spanned<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Let {
        definition: Box<Spanned<Self>>,
        expression: Box<Spanned<Self>>,
    },
    Perform {
        effect: Box<Spanned<Self>>,
    },
    Effectful {
        class: Spanned<Type>,
        expr: Box<Spanned<Self>>,
        handlers: Vec<Handler>,
    },
    Call {
        function: Spanned<Type>,
        uuid: Option<Uuid>,
        arguments: Vec<Spanned<Self>>,
    },
    Product(Vec<Spanned<Self>>),
    Typed {
        ty: Spanned<Type>,
        expr: Box<Spanned<Self>>,
    },
    Hole,
    Function {
        parameters: Vec<Spanned<Type>>,
        body: Box<Spanned<Self>>,
    },
    Array(Vec<Spanned<Self>>),
    Set(Vec<Spanned<Self>>),
    Module(Box<Spanned<Self>>),
    Import {
        ty: Spanned<Type>,
        uuid: Option<Uuid>,
    },
    Export {
        ty: Spanned<Type>,
        uuid: Option<Uuid>,
    },
}

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
        let literal = rational
            .or(int64.map(|int| Expr::Literal(Literal::Int(int))))
            .or(filter_map(|span, token| match token {
                Token::Str(string) => Ok(Expr::Literal(Literal::String(string))),
                _ => Err(Simple::custom(span, "expected string literal")),
            }));
        let type_ = r#type::parser().delimited_by(Token::TypeBegin, Token::TypeEnd);
        let let_ = just(Token::Let)
            .ignore_then(expr.clone())
            .then(
                just(Token::TypeAnnotation)
                    .ignore_then(type_.clone())
                    .or_not(),
            )
            .then_ignore(just(Token::In).or_not())
            .then(expr.clone())
            .map_with_span(|((definition, type_), expression), span| Expr::Let {
                definition: Box::new(if let Some(type_) = type_ {
                    (
                        Expr::Typed {
                            ty: type_,
                            expr: Box::new(definition),
                        },
                        span,
                    )
                } else {
                    definition
                }),
                expression: Box::new(expression),
            });
        let perform = just(Token::Perform)
            .ignore_then(expr.clone())
            .map(|effect| Expr::Perform {
                effect: Box::new(effect),
            });
        let effectful =
            parse_effectful(expr.clone(), type_.clone()).map(|(class, expr, handlers)| {
                Expr::Effectful {
                    class,
                    expr: Box::new(expr),
                    handlers: handlers
                        .into_iter()
                        .map(|handler| Handler {
                            ty: handler.0,
                            expr: handler.1,
                        })
                        .collect(),
                }
            });
        let call = type_
            .clone()
            .then(
                just(Token::Uuid)
                    .ignore_then(filter_map(|span, token| {
                        if let Token::Ident(uuid) = token {
                            Ok(uuid.parse().map_err(|e| {
                                dbg!(Simple::custom(
                                    span,
                                    format!("failed to parse uuid: {}, {}", uuid, e),
                                ))
                            })?)
                        } else {
                            Err(Simple::custom(span, "expected uuid"))
                        }
                    }))
                    .or_not(),
            )
            .then(expr.clone().separated_by(just(Token::Comma)))
            .map(|((function, uuid), arguments)| Expr::Call {
                function,
                uuid,
                arguments,
            })
            .dot();
        let product =
            parse_op(just(Token::Product), expr.clone()).map(|values| Expr::Product(values));
        let function = parse_function(just(Token::Lambda), type_, just(Token::Arrow), expr.clone())
            .map(|(parameters, body)| Expr::Function {
                parameters,
                body: Box::new(body),
            });
		let array = parse_collection(Token::ArrayBegin, expr.clone(), Token::ArrayEnd).map(Expr::Array);
		let set = parse_collection(Token::SetBegin, expr.clone(), Token::SetEnd).map(Expr::Set);
        hole.or(literal)
            .or(let_)
            .or(perform)
            .or(effectful)
            .or(call)
            .or(product)
            .or(function)
            .or(array)
            .or(set)
            .map_with_span(|token, span| (token, span))
    })
}

#[cfg(test)]
mod tests {
    use matches::assert_matches;

    use chumsky::Stream;

    use crate::lexer::lexer;

    use super::*;

    fn parse(input: &str) -> Result<Spanned<Expr>, Vec<Simple<Token>>> {
        parser().parse(Stream::from_iter(
            input.len()..input.len() + 1,
            dbg!(lexer().then_ignore(end()).parse(input).unwrap().into_iter()),
        ))
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
        assert_matches!(
            parse("$ 3: <'number> ~ ?.").unwrap().0,
            Expr::Let {
                definition: box (
                    Expr::Typed {
                        expr: box (Expr::Literal(Literal::Int(3)), _),
                        ty: (Type::Number, _)
                    },
                    _
                ),
                expression: box (Expr::Hole, _),
            }
        );
    }

    #[test]
    fn parse_let_without_type_in_and_dot() {
        assert_matches!(
            parse("$ 3 ?").unwrap().0,
            Expr::Let {
                definition: box (Expr::Literal(Literal::Int(3)), _),
                expression: box (Expr::Hole, _),
            }
        );
    }

    #[test]
    fn parse_perform() {
        assert_matches!(
            parse("! ?").unwrap().0,
            Expr::Perform {
                effect: box (Expr::Hole, _),
            }
        );
    }

    #[test]
    fn parse_handle() {
        let trait_ = parse(r#"# <'a class> ?; <'a num_to_num> => ?, <'a str_to_str> => ?"#)
            .unwrap()
            .0;
        assert_eq!(
            trait_,
            Expr::Effectful {
                class: (Type::Alias("class".into()), 3..11),
                expr: Box::new((Expr::Hole, 13..14)),
                handlers: vec![
                    Handler {
                        ty: (Type::Alias("num_to_num".into()), 17..30),
                        expr: (Expr::Hole, 35..36),
                    },
                    Handler {
                        ty: (Type::Alias("str_to_str".into()), 39..52),
                        expr: (Expr::Hole, 57..58),
                    }
                ]
            }
        );
    }

    #[test]
    fn parse_call() {
        assert_eq!(
            parse("<'a add> 1, 2.").unwrap().0,
            Expr::Call {
                function: (Type::Alias("add".into()), 1..7),
                uuid: None,
                arguments: vec![
                    (Expr::Literal(Literal::Int(1)), 9..10),
                    (Expr::Literal(Literal::Int(2)), 12..13)
                ],
            }
        );
    }

    #[test]
    fn parse_call_with_uuid() {
        assert_eq!(
            parse("<'a add> 'uuid ee525632-9506-4926-aec0-36cbfe65ac0f 1, 2")
                .unwrap()
                .0,
            Expr::Call {
                function: (Type::Alias("add".into()), 1..7),
                uuid: Some("ee525632-9506-4926-aec0-36cbfe65ac0f".parse().unwrap()),
                arguments: vec![
                    (Expr::Literal(Literal::Int(1)), 52..53),
                    (Expr::Literal(Literal::Int(2)), 55..56)
                ],
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
            parse(r#"\ <'number>, <'number> -> ?"#).unwrap().0,
            Expr::Function {
                parameters: vec![(Type::Number, 3..10), (Type::Number, 14..21)],
                body: Box::new((Expr::Hole, 26..27)),
            }
        );
    }

	#[test]
	fn parse_array() {
		assert_eq!(
			parse("[1, ?, ?]").unwrap().0,
			Expr::Array(vec![
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
}
