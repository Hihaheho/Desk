mod error;

use std::ops::Range;

use chumsky::prelude::*;
use error::LexerError;
use tokens::Token;
use uuid::Uuid;

pub fn scan(input: &str) -> Result<Vec<(Token, Range<usize>)>, LexerError> {
    lexer().then_ignore(end()).parse(input).map_err(LexerError)
}

pub fn lexer() -> impl Parser<char, Vec<(Token, Range<usize>)>, Error = Simple<char>> {
    let comment = recursive(|comment| {
        none_of("()")
            .repeated()
            .at_least(1)
            .collect::<String>()
            .or(comment)
            .repeated()
            .delimited_by(just('('), just(')'))
            .map(|s| format!("({})", s.join("")))
    })
    .map(Token::Comment);
    let identifier = ident().map(Token::Ident);
    let int = just('-')
        .or_not()
        .chain::<char, _, _>(text::int(10))
        .collect::<String>()
        .map(|s: String| Token::Int(s.parse().unwrap()));
    let escape = just('\\').ignore_then(
        just('\\')
            .or(just('"'))
            .or(just('n').to('\n'))
            .or(just('r').to('\r'))
            .or(just('t').to('\t')),
    );
    let string = just('"')
        .ignore_then(filter(|c| *c != '\\' && *c != '"').or(escape).repeated())
        .then_ignore(just('"'))
        .collect::<String>()
        .map(|s| {
            s.replace(r#"\\"#, r#"\"#)
                .replace(r#"\""#, "\"")
                .replace(r#"\n"#, "\n")
                .replace(r#"\r"#, "\r")
                .replace(r#"\t"#, "\t")
        })
        .map(Token::Str);
    let symbol = just('$')
        .to(Token::Let)
        .or(just('~').to(Token::In))
        .or(just('/').to(Token::Divide))
        .or(just('!').to(Token::Perform))
        .or(just(':').to(Token::TypeAnnotation))
        .or(just('%').to(Token::Trait))
        .or(just('#').to(Token::Attribute))
        .or(just('^').to(Token::FromHere))
        .or(just('+').to(Token::Sum))
        .or(just('*').to(Token::Product))
        .or(just(',').to(Token::Comma))
        .or(just('.').to(Token::Dot))
        .or(just('>').to(Token::Apply))
        .or(just('[').to(Token::ArrayBegin))
        .or(just(']').to(Token::ArrayEnd))
        .or(just('{').to(Token::SetBegin))
        .or(just('}').to(Token::SetEnd))
        .or(just('?').to(Token::Hole))
        .or(just('_').to(Token::Infer))
        .or(just('\\').to(Token::Lambda))
        .or(just('&').to(Token::Reference))
        .or(just('<').chain(just('!')).to(Token::Continue))
        .or(just('-').chain(just('>')).to(Token::Arrow))
        .or(just('=').chain(just('>')).to(Token::EArrow))
        .or(just('-').to(Token::Minus));
    let special = just('\'')
        .ignore_then(text::ident())
        .try_map(|ident: String, span| match ident.as_str() {
            "module" => Ok(Token::Include),
            "import" => Ok(Token::Import),
            "export" => Ok(Token::Export),
            "number" => Ok(Token::NumberType),
            "string" => Ok(Token::StringType),
            "alias" => Ok(Token::Alias),
            "brand" => Ok(Token::Brands),
            "type" => Ok(Token::Type),
            "this" => Ok(Token::This),
            "handle" => Ok(Token::Handle),
            "card" => Ok(Token::Card),
            "a" => Ok(Token::A),
            _ => Err(Simple::custom(
                span,
                format!(r#"undefined special keyword: {}"#, ident),
            )),
        });
    let brand = just('@')
        .ignore_then(ident())
        .map(|ident: String| Token::Brand(ident.into()));
    let uuid = just("'uuid")
        .then_ignore(text::whitespace())
        .ignore_then(
            one_of("0123456789abcdefABCDEF")
                .repeated()
                .at_least(4)
                .separated_by(just('-'))
                .at_least(1),
        )
        .flatten()
        .collect::<String>()
        .map(|uuid| uuid.parse::<Uuid>())
        .try_map(|uuid, span| match uuid {
            Ok(uuid) => Ok(Token::Uuid(uuid)),
            Err(_) => Err(Simple::custom(span, "invalid uuid")),
        });
    let token = comment
        .or(uuid)
        .or(int)
        .or(string)
        .or(symbol)
        .or(special)
        .or(brand)
        .or(identifier);
    let semicolon = just(';').to(()).map_with_span(|_, span: Range<usize>| {
        vec![(Token::Dot, span.clone()), (Token::Comma, span)]
    });
    token
        .map_with_span(|token, span| (token, span))
        .padded()
        .repeated()
        .at_least(1)
        .or(semicolon)
        .repeated()
        .flatten()
}

pub fn ident() -> impl Parser<char, String, Error = Simple<char>> + Clone {
    let assert_not_whitespace = |c: char, span| {
        if c.is_whitespace() {
            Err(Simple::custom(span, "invalid character"))
        } else {
            Ok(c)
        }
    };
    let non_symbol =
        none_of(r#"%@/&$<>!#*^?\[]{}_-+=;:~,.()"'1234567890"#).try_map(assert_not_whitespace);
    // Does not have @, underscore, hyphen, and single quote.
    let non_symbol_2 = none_of(r#"%/&$<>!#*^?\[]{}+=;:~,.()"#).try_map(assert_not_whitespace);

    non_symbol
        .chain::<char, _, _>(non_symbol_2.repeated())
        .collect()
        .separated_by(text::whitespace())
        .at_least(1)
        .map(|ident: Vec<String>| ident.join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_syntax() {
        let tokens = lexer()
            .parse(
                r#"
            (defines < 'number -> @incremented 'number >)
            $ \x -> ^ \ 'number, 'number -> @added 'number >
                1, x (1 + x): @incremented 'number > ~
            (increments a value which is padded later)
            \ 'number -> @incremented 'number > ?.
            "#,
            )
            .unwrap()
            .into_iter()
            .map(|(token, _)| token)
            .collect::<Vec<_>>();

        use Token::*;
        assert_eq!(
            tokens,
            vec![
                Comment("(defines < 'number -> @incremented 'number >)".into()),
                Let,
                Lambda,
                Ident("x".into()),
                Arrow,
                FromHere,
                Lambda,
                NumberType,
                Comma,
                NumberType,
                Arrow,
                Brand("added".into()),
                NumberType,
                Apply,
                Int(1),
                Comma,
                Ident("x".into()),
                Comment("(1 + x)".into()),
                TypeAnnotation,
                Brand("incremented".into()),
                NumberType,
                Apply,
                In,
                Comment("(increments a value which is padded later)".into()),
                Lambda,
                NumberType,
                Arrow,
                Brand("incremented".into()),
                NumberType,
                Apply,
                Hole,
                Dot,
            ]
        )
    }

    #[test]
    fn ident_with_spaces() {
        assert_eq!(
            lexer().parse(" the\t\nnumber  of apples ").unwrap(),
            vec![(Token::Ident("the number of apples".into()), 1..23)]
        );
    }

    #[test]
    fn ident_utf8() {
        assert_eq!(
            lexer().parse("あ-　a0").unwrap(),
            vec![(Token::Ident("あ- a0".into()), 0..5)]
        );
    }

    #[test]
    fn brand_with_spaces() {
        assert_eq!(
            lexer().parse("@あ-　a0").unwrap(),
            vec![(Token::Brand("あ- a0".into()), 0..6)]
        );
    }

    #[test]
    fn string_with_escape() {
        assert_eq!(
            lexer()
                .parse(
                    r#"
            "\\\n\""
            "#
                )
                .unwrap(),
            vec![(Token::Str("\\\n\"".into()), 13..21)]
        );
    }

    #[test]
    fn semicolon_to_comma_dot() {
        assert_eq!(
            lexer().then_ignore(end()).parse("?;?").unwrap(),
            vec![
                (Token::Hole, 0..1),
                (Token::Dot, 1..2),
                (Token::Comma, 1..2),
                (Token::Hole, 2..3)
            ]
        );
    }
}
