use chumsky::prelude::*;
use uuid::Uuid;

use crate::{lexer::Token, span::Spanned};

use super::{
    common::parse_effectful,
    r#type::{self, Type},
};

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
    Product {
        values: Vec<Spanned<Self>>,
    },
    Typed {
        ty: Spanned<Type>,
        expr: Box<Spanned<Self>>,
    },
    Hole,
    Function {
        parameters: Vec<Spanned<Type>>,
        expression: Box<Spanned<Self>>,
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
        hole.or(literal)
            .or(let_)
            .or(perform)
            .or(effectful)
            .then_ignore(just(Token::Dot).or_not())
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
            dbg!(lexer().parse(input).unwrap().into_iter()),
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
}
