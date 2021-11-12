use chumsky::prelude::*;

use crate::span::Spanned;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Token {
    Comment(String),
    Ident(String),
    Int(i64),
    Str(String),
    // TODO: Float(i64, i64),
    Uuid,
    Divide,
    Let,
    In,
    Perform,
    WithHandler,
    This,
    FromHere,
    TypeAnnotation,
    Trait,
    Effectful,
    Sum,
    Product,
    Comma,
    Dot,
    CommentBegin,
    CommentEnd,
    TypeBegin,
    TypeEnd,
    ArrayBegin,
    ArrayEnd,
    SetBegin,
    SetEnd,
    Hole,
    Infer,
    Lambda,
    Arrow,
    EArrow,
    Module,
    Import,
    Export,
    Continue,
    Private,
    Type,
    NumberType,
    StringType,
    Brand(String),
    Alias,
    A,
}

pub fn lexer() -> impl Parser<char, Vec<Spanned<Token>>, Error = Simple<char>> {
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
        .map(|ident: Vec<String>| dbg!(Token::Ident(ident.join(" "))));
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
        .or(just(';').to(Token::WithHandler))
        .or(just(':').to(Token::TypeAnnotation))
        .or(just('%').to(Token::Trait))
        .or(just('#').to(Token::Effectful))
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
            "uuid" => Ok(Token::Uuid),
            "module" => Ok(Token::Module),
            "import" => Ok(Token::Import),
            "export" => Ok(Token::Export),
            "number" => Ok(Token::NumberType),
            "string" => Ok(Token::StringType),
            "alias" => Ok(Token::Alias),
            "type" => Ok(Token::Type),
            "private" => Ok(Token::Private),
            "continue" => Ok(Token::Continue),
            "a" => Ok(Token::A),
            _ => Err(Simple::custom(
                span,
                format!(r#"undefined special keyword: {}"#, ident),
            )),
        });
    let brand = just('@')
        .ignore_then(ident())
        .map(|ident: String| Token::Brand(ident.into()));
    let token = comment
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
    none_of(r#"/$<>!#*^?\[]{}-=;:~,.()'"#.chars())
        .try_map(|c, span| {
            if c.is_whitespace() || c.is_numeric() {
                Err(Simple::custom(span, "invalid character"))
            } else {
                Ok(c)
            }
        })
        .map(Some)
        .chain::<char, _, _>(
            // Does not have hyphen.
            none_of(r#"/$<>!#*^?\[]{}=;:~,.()'"#.chars())
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
            vec![(Token::Ident("あ- a0".into()), 0..3)]
        );
    }
}
