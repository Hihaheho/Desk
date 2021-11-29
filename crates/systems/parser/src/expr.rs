use ast::{
    expr::{Expr, Literal, MatchCase},
    span::Spanned,
    ty::Type,
};
use chumsky::prelude::*;
use tokens::Token;

use crate::common::parse_attr;

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
        let type_ = super::ty::parser(expr.clone())
            .delimited_by(Token::TypeBegin, Token::TypeEnd)
            .or(super::ty::parser(expr.clone()));
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
                expression: Box::new(expression),
            });
        let perform = just(Token::Perform)
            .ignore_then(expr.clone())
            .then_ignore(just(Token::EArrow))
            .then(type_.clone())
            .map(|(effect, output)| Expr::Perform {
                input: Box::new(effect),
                output,
            });
        let handle = type_
            .clone()
            .then_ignore(just(Token::EArrow))
            .then(type_.clone())
            .then(expr.clone().in_().then(expr.clone()))
            .map(|((input, output), (handler, expr))| Expr::Handle {
                input,
                output,
                handler: Box::new(handler),
                expr: Box::new(expr),
            });
        let apply =
            type_
                .clone()
                .then(expr.clone().separated_by_comma())
                .map(|(function, arguments)| Expr::Apply {
                    function,
                    arguments,
                });
        let product =
            parse_op(just(Token::Product), expr.clone()).map(|values| Expr::Product(values));
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
            parse_collection(Token::ArrayBegin, expr.clone(), Token::ArrayEnd).map(Expr::Array);
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
        let ident = filter_map(|span, token| match token {
            Token::Ident(ident) => Ok(ident),
            _ => Err(Simple::custom(span, "expected identifier")),
        });
        let brand = just(Token::Brands)
            .ignore_then(ident.separated_by_comma())
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
                    .separated_by_comma(),
            )
            .map(|(of, cases)| Expr::Match {
                of: Box::new(of),
                cases,
            });
        let include = just(Token::Include)
            .ignore_then(ident.clone())
            .map(|ident| Expr::Include(ident));
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
            .ignore_then(ident.clone())
            .then(type_.clone())
            .in_()
            .then(expr.clone())
            .map(|((ident, ty), expr)| Expr::NewType {
                ident,
                ty,
                expr: Box::new(expr),
            });

        hole.or(literal)
            .or(let_in)
            .or(perform)
            .or(handle)
            .or(product)
            .or(function)
            .or(array)
            .or(set)
            .or(typed)
            .or(attribute)
            .or(brand)
            .or(match_)
            .or(include)
            .or(label)
            .or(apply)
            .or(newtype)
            .then_ignore(none_of([Token::Arrow]).to(()).or(end()).lookahead())
            .map_with_span(|token, span| (token, span))
    })
}

#[cfg(test)]
mod tests {
    use ast::{expr::MatchCase, ty::Type};
    use lexer::scan;

    use super::*;

    fn parse(input: &str) -> Result<Spanned<Expr>, Vec<Simple<Token>>> {
        crate::parse(dbg!(scan(input).unwrap()))
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
            parse("$ 3: <'number> ~ ?").unwrap().0,
            Expr::Let {
                ty: (Type::Number, 6..13),
                definition: Box::new((Expr::Literal(Literal::Int(3)), 2..3)),
                expression: Box::new((Expr::Hole, 17..18)),
            }
        );
    }

    #[test]
    fn parse_perform() {
        assert_eq!(
            parse("! ? => <'string>").unwrap().0,
            Expr::Perform {
                input: Box::new((Expr::Hole, 2..3)),
                output: (Type::String, 8..15),
            }
        );
    }

    #[test]
    fn parse_handle() {
        let trait_ = parse(r#"<'number> => <'string> 3 ~ ?"#).unwrap().0;
        assert_eq!(
            trait_,
            Expr::Handle {
                input: (Type::Number, 1..8),
                output: (Type::String, 14..21),
                handler: Box::new((Expr::Literal(Literal::Int(3)), 23..24)),
                expr: Box::new((Expr::Hole, 27..28)),
            }
        );
    }

    #[test]
    fn parse_call() {
        assert_eq!(
            parse("<'a add> 1, 2.").unwrap().0,
            Expr::Apply {
                function: (Type::Alias("add".into()), 1..7),
                arguments: vec![
                    (Expr::Literal(Literal::Int(1)), 9..10),
                    (Expr::Literal(Literal::Int(2)), 12..13)
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
            parse(r#"\ <'number>, <_> -> ?"#).unwrap().0,
            Expr::Function {
                parameters: vec![(Type::Number, 3..10), (Type::Infer, 14..15)],
                body: Box::new((Expr::Hole, 20..21)),
            },
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

    #[test]
    fn parse_type_annotation() {
        assert_eq!(
            parse("^?: <'number>").unwrap().0,
            Expr::Typed {
                item: Box::new((Expr::Hole, 1..2)),
                ty: (Type::Number, 5..12),
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
            <'number> -> "number",
            <'string> -> "string".
            "#
            )
            .unwrap()
            .0,
            Expr::Match {
                of: Box::new((Expr::Hole, 15..16)),
                cases: vec![
                    MatchCase {
                        ty: (Type::Number, 32..39),
                        expr: (Expr::Literal(Literal::String("number".into())), 44..52),
                    },
                    MatchCase {
                        ty: (Type::String, 67..74),
                        expr: (Expr::Literal(Literal::String("string".into())), 79..87),
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
            + 'a x
            <'number> -> "number",
            <'string> -> "string".
            "#
            )
            .unwrap()
            .0,
            Expr::Match {
                of: Box::new((
                    Expr::Apply {
                        function: (Type::Alias("x".into()), 15..19),
                        arguments: vec![]
                    },
                    15..19
                )),
                cases: vec![
                    MatchCase {
                        ty: (Type::Number, 33..40),
                        expr: (Expr::Literal(Literal::String("number".into())), 45..53),
                    },
                    MatchCase {
                        ty: (Type::String, 68..75),
                        expr: (Expr::Literal(Literal::String("string".into())), 80..88),
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
}
