use std::ops::Range;

use chumsky::prelude::*;
use tokens::Token;
use uuid::Uuid;

pub fn scan(input: &str) -> Result<Vec<(Token, Range<usize>)>, Vec<Simple<char>>> {
    lexer().then_ignore(end()).parse(input)
}

pub fn lexer() -> impl Parser<char, Vec<(Token, Range<usize>)>, Error = Simple<char>> {
    let comment = recursive(|comment| {
        none_of("()".chars())
            .repeated()
            .collect::<String>()
            .or(comment)
            .delimited_by('(', ')')
            .map(|s| format!("({})", s))
    })
    .map(Token::Comment);
    let identifier = ident()
        .separated_by(text::whitespace())
        .at_least(1)
        .map(|ident: Vec<String>| Token::Ident(ident.join(" ")));
    let int = just('-')
        .or_not()
        .chain::<char, _, _>(text::int(10))
        .collect::<String>()
        .map(|s: String| Token::Int(s.parse().unwrap()));
    let string = just('"')
        .ignore_then(none_of(['"']).repeated())
        .then_ignore(just('"'))
        .collect::<String>()
        .map(Token::Str);
    let symbol = just('$')
        .to(Token::Let)
        .or(just('~').to(Token::In))
        .or(just('/').to(Token::Divide))
        .or(just('!').to(Token::Perform))
        .or(just(':').to(Token::TypeAnnotation))
        .or(just('%').to(Token::Trait))
        .or(just('#').to(Token::Attribute))
        .or(just('&').to(Token::This))
        .or(just('^').to(Token::FromHere))
        .or(just('+').to(Token::Sum))
        .or(just('*').to(Token::Product))
        .or(just(',').to(Token::Comma))
        .or(just('.').to(Token::Dot))
        .or(just('(').to(Token::CommentBegin))
        .or(just(')').to(Token::CommentEnd))
        .or(just('<').to(Token::TypeBegin))
        .or(just('>').to(Token::TypeEnd))
        .or(just('[').to(Token::ArrayBegin))
        .or(just(']').to(Token::ArrayEnd))
        .or(just('{').to(Token::SetBegin))
        .or(just('}').to(Token::SetEnd))
        .or(just('?').to(Token::Hole))
        .or(just('_').to(Token::Infer))
        .or(just('\\').to(Token::Lambda))
        .or(just('-').chain(just('>')).to(Token::Arrow))
        .or(just('=').chain(just('>')).to(Token::EArrow));
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
            "a" => Ok(Token::A),
            _ => Err(Simple::custom(
                span,
                format!(r#"undefined special keyword: {}"#, ident),
            )),
        });
    let brand = just('@')
        .ignore_then(ident())
        .map(|ident: String| Token::Brand(ident.into()));
    let uuid = seq("'uuid".chars())
        .chain(text::whitespace())
        .ignore_then(
            one_of("0123456789abcdefABCDEF".chars())
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
    token
        .map_with_span(|token, span| (token, span))
        .padded()
        .repeated()
}

pub fn ident() -> impl Parser<char, String, Error = Simple<char>> + Clone {
    "a".contains("a");
    none_of(r#"%@/&$<>!#*^?\[]{}_-=;:~,.()'"#.chars())
        .try_map(|c, span| {
            if c.is_whitespace() {
                Err(Simple::custom(span, "invalid character"))
            } else {
                Ok(c)
            }
        })
        .map(Some)
        .chain::<char, _, _>(
            // Does not have @, underscore, hyphen, and single quote.
            none_of(r#"%/&$<>!#*^?\[]{}=;:~,.()"#.chars())
                .try_map(|c, span| {
                    if c.is_whitespace() {
                        Err(Simple::custom(span, "invalid character"))
                    } else {
                        Ok(c)
                    }
                })
                .repeated(),
        )
        .collect()
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
            $ \x -> ^ <\ 'number, 'number -> @added 'number >
                1, x (1 + x): < @incremented 'number > ~
            (increments a value which is padded later)
            <\ 'number -> @incremented 'number > ?.
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
                TypeBegin,
                Lambda,
                NumberType,
                Comma,
                NumberType,
                Arrow,
                Brand("added".into()),
                NumberType,
                TypeEnd,
                Int(1),
                Comma,
                Ident("x".into()),
                Comment("(1 + x)".into()),
                TypeAnnotation,
                TypeBegin,
                Brand("incremented".into()),
                NumberType,
                TypeEnd,
                In,
                Comment("(increments a value which is padded later)".into()),
                TypeBegin,
                Lambda,
                NumberType,
                Arrow,
                Brand("incremented".into()),
                NumberType,
                TypeEnd,
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
}
