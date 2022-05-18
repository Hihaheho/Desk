use ast::span::WithSpan;
use chumsky::{
    combinator::{Map, OrNot, SeparatedBy, Then},
    prelude::*,
    primitive::Just,
    Error,
};
use tokens::Token;
use uuid::Uuid;

pub(crate) fn parse_comment() -> impl Parser<Token, String, Error = Simple<Token>> + Clone {
    filter_map(|span, token| match token {
        Token::Comment(text) => Ok(text),
        _ => Err(Simple::custom(span, "expected comment")),
    })
}

pub(crate) fn parse_ident() -> impl Parser<Token, String, Error = Simple<Token>> + Clone {
    filter_map(|span, token| match token {
        Token::Ident(ident) => Ok(ident),
        _ => Err(Simple::custom(span, "expected identifier")),
    })
}

pub(crate) fn parse_uuid() -> impl Parser<Token, Uuid, Error = Simple<Token>> + Clone {
    filter_map(|span, token| match token {
        Token::Uuid(ident) => Ok(ident),
        _ => Err(Simple::custom(span, "expected uuid")),
    })
}

pub(crate) fn parse_op<U, O>(
    op: impl Parser<Token, U, Error = Simple<Token>> + Clone,
    item: impl Parser<Token, WithSpan<O>, Error = Simple<Token>> + Clone,
) -> impl Parser<Token, Vec<WithSpan<O>>, Error = Simple<Token>> + Clone {
    op.ignore_then(item.separated_by_comma())
}

pub(crate) fn parse_function<A, O, U>(
    op: impl Parser<Token, U, Error = Simple<Token>> + Clone,
    args: impl Parser<Token, WithSpan<A>, Error = Simple<Token>> + Clone,
    arrow: impl Parser<Token, U, Error = Simple<Token>> + Clone,
    output: impl Parser<Token, WithSpan<O>, Error = Simple<Token>> + Clone,
) -> impl Parser<Token, (Vec<WithSpan<A>>, WithSpan<O>), Error = Simple<Token>> + Clone {
    op.ignore_then(args.separated_by(just(Token::Comma)))
        .then_ignore(arrow)
        .then(output)
}

pub(crate) fn parse_collection<T>(
    begin: Token,
    item: impl Parser<Token, WithSpan<T>, Error = Simple<Token>> + Clone,
    end: Token,
) -> impl Parser<Token, Vec<WithSpan<T>>, Error = Simple<Token>> + Clone {
    item.separated_by(just(Token::Comma))
        .delimited_by(just(begin), just(end))
}

pub(crate) fn parse_typed<I, T>(
    item: impl Parser<Token, WithSpan<I>, Error = Simple<Token>> + Clone,
    ty: impl Parser<Token, WithSpan<T>, Error = Simple<Token>> + Clone,
) -> impl Parser<Token, (WithSpan<I>, WithSpan<T>), Error = Simple<Token>> + Clone {
    just(Token::FromHere)
        .ignore_then(item)
        .then_ignore(just(Token::TypeAnnotation))
        .then(ty)
}

pub(crate) fn parse_attr<I, T>(
    attr: impl Parser<Token, WithSpan<I>, Error = Simple<Token>> + Clone,
    item: impl Parser<Token, WithSpan<T>, Error = Simple<Token>> + Clone,
) -> impl Parser<Token, (WithSpan<I>, WithSpan<T>), Error = Simple<Token>> + Clone {
    just(Token::Attribute).ignore_then(attr).in_().then(item)
}

type In<T, E, O> =
    Map<Then<T, OrNot<Just<Token, Token, E>>>, fn((O, Option<Token>)) -> O, (O, Option<Token>)>;

type SeparatedByComma<T, E, O> = Map<
    OrNot<
        Map<
            Then<SeparatedBy<T, Just<Token, Token, E>, Token>, OrNot<Just<Token, Token, E>>>,
            fn((Vec<O>, Option<Token>)) -> Vec<O>,
            (Vec<O>, Option<Token>),
        >,
    >,
    fn(Option<Vec<O>>) -> Vec<O>,
    Option<Vec<O>>,
>;

type SeparatedByCommaAtLeastOne<T, E, O> = Map<
    Then<SeparatedBy<T, Just<Token, Token, E>, Token>, OrNot<Just<Token, Token, E>>>,
    fn((Vec<O>, Option<Token>)) -> Vec<O>,
    (Vec<O>, Option<Token>),
>;

pub(crate) trait ParserExt<O>
where
    Self: Parser<Token, O> + Sized,
{
    fn in_(self) -> In<Self, Self::Error, O>;

    fn separated_by_comma(self) -> SeparatedByComma<Self, Self::Error, O>;

    fn separated_by_comma_at_least_one(self) -> SeparatedByCommaAtLeastOne<Self, Self::Error, O>;
}

impl<T: Parser<Token, O, Error = E>, O, E: Error<Token>> ParserExt<O> for T {
    fn in_(self) -> In<Self, Self::Error, O>
    where
        Self: Sized,
    {
        self.then_ignore(just(Token::In).or_not())
    }

    fn separated_by_comma(self) -> SeparatedByComma<Self, Self::Error, O> {
        self.separated_by_comma_at_least_one()
            .or_not()
            .map(|option| option.unwrap_or_default())
    }

    fn separated_by_comma_at_least_one(self) -> SeparatedByCommaAtLeastOne<Self, Self::Error, O> {
        self.separated_by(just(Token::Comma))
            .at_least(1)
            .then_ignore(just(Token::Dot).or_not())
    }
}
