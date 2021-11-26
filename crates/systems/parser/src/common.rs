use ast::span::Spanned;
use chumsky::{
    combinator::{Map, OrNot, SeparatedBy, Then},
    prelude::*,
    primitive::Just,
    Error,
};
use tokens::Token;

pub(crate) fn parse_op<U, O>(
    op: impl Parser<Token, U, Error = Simple<Token>> + Clone,
    item: impl Parser<Token, Spanned<O>, Error = Simple<Token>> + Clone,
) -> impl Parser<Token, Vec<Spanned<O>>, Error = Simple<Token>> + Clone {
    op.ignore_then(item.separated_by_comma())
}

pub(crate) fn parse_function<A, O, U>(
    op: impl Parser<Token, U, Error = Simple<Token>> + Clone,
    args: impl Parser<Token, Spanned<A>, Error = Simple<Token>> + Clone,
    arrow: impl Parser<Token, U, Error = Simple<Token>> + Clone,
    output: impl Parser<Token, Spanned<O>, Error = Simple<Token>> + Clone,
) -> impl Parser<Token, (Vec<Spanned<A>>, Spanned<O>), Error = Simple<Token>> + Clone {
    op.ignore_then(args.separated_by(just(Token::Comma)))
        .then_ignore(arrow)
        .then(output.clone())
}

pub(crate) fn parse_collection<T>(
    begin: Token,
    item: impl Parser<Token, Spanned<T>, Error = Simple<Token>> + Clone,
    end: Token,
) -> impl Parser<Token, Vec<Spanned<T>>, Error = Simple<Token>> + Clone {
    item.separated_by(just(Token::Comma))
        .delimited_by(begin, end)
}

pub(crate) fn parse_typed<I, T>(
    item: impl Parser<Token, Spanned<I>, Error = Simple<Token>> + Clone,
    ty: impl Parser<Token, Spanned<T>, Error = Simple<Token>> + Clone,
) -> impl Parser<Token, (Spanned<I>, Spanned<T>), Error = Simple<Token>> + Clone {
    just(Token::FromHere)
        .ignore_then(item)
        .then_ignore(just(Token::TypeAnnotation))
        .then(ty)
}

pub(crate) fn parse_attr<I, T>(
    attr: impl Parser<Token, Spanned<I>, Error = Simple<Token>>,
    item: impl Parser<Token, Spanned<T>, Error = Simple<Token>>,
) -> impl Parser<Token, (Spanned<I>, Spanned<T>), Error = Simple<Token>> {
    just(Token::Attribute).ignore_then(attr).in_().then(item)
}

pub(crate) trait ParserExt<O>
where
    Self: Parser<Token, O> + Sized,
{
    fn in_(
        self,
    ) -> Map<
        Then<Self, OrNot<Just<Token, Self::Error>>>,
        fn((O, Option<Token>)) -> O,
        (O, Option<Token>),
    >;

    fn separated_by_comma(
        self,
    ) -> Map<
        OrNot<
            Map<
                Then<
                    SeparatedBy<Self, Just<Token, Self::Error>, Token>,
                    OrNot<Just<Token, Self::Error>>,
                >,
                fn((Vec<O>, Option<Token>)) -> Vec<O>,
                (Vec<O>, Option<Token>),
            >,
        >,
        fn(Option<Vec<O>>) -> Vec<O>,
        Option<Vec<O>>,
    >;

    fn separated_by_comma_at_least_one(
        self,
    ) -> Map<
        Then<SeparatedBy<Self, Just<Token, Self::Error>, Token>, OrNot<Just<Token, Self::Error>>>,
        fn((Vec<O>, Option<Token>)) -> Vec<O>,
        (Vec<O>, Option<Token>),
    >;
}

impl<T: Parser<Token, O, Error = E>, O, E: Error<Token>> ParserExt<O> for T {
    fn separated_by_comma(
        self,
    ) -> Map<
        OrNot<
            Map<
                Then<
                    SeparatedBy<T, Just<Token, Self::Error>, Token>,
                    OrNot<Just<Token, Self::Error>>,
                >,
                fn((Vec<O>, Option<Token>)) -> Vec<O>,
                (Vec<O>, Option<Token>),
            >,
        >,
        fn(Option<Vec<O>>) -> Vec<O>,
        Option<Vec<O>>,
    > {
        self.separated_by_comma_at_least_one()
            .or_not()
            .map(|option| option.unwrap_or(vec![]))
    }

    fn separated_by_comma_at_least_one(
        self,
    ) -> Map<
        Then<SeparatedBy<T, Just<Token, Self::Error>, Token>, OrNot<Just<Token, Self::Error>>>,
        fn((Vec<O>, Option<Token>)) -> Vec<O>,
        (Vec<O>, Option<Token>),
    > {
        self.separated_by(just(Token::Comma))
            .at_least(1)
            .then_ignore(just(Token::Dot).or_not())
    }

    fn in_(
        self,
    ) -> Map<
        Then<Self, OrNot<Just<Token, Self::Error>>>,
        fn((O, Option<Token>)) -> O,
        (O, Option<Token>),
    >
    where
        Self: Sized,
    {
        self.then_ignore(just(Token::In).or_not())
    }
}
