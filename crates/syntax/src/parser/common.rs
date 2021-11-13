use chumsky::{
    combinator::{Map, OrNot, Then},
    prelude::*,
    primitive::Just,
    Error,
};

use crate::{lexer::Token, span::Spanned};

pub(crate) fn parse_effectful<I, T>(
    item: impl Parser<Token, Spanned<I>, Error = Simple<Token>> + Clone,
    ty: impl Parser<Token, Spanned<T>, Error = Simple<Token>> + Clone,
) -> impl Parser<Token, (Spanned<T>, Spanned<I>, Vec<(Spanned<T>, Spanned<I>)>), Error = Simple<Token>>
       + Clone {
    just(Token::Effectful)
        .ignore_then(ty.clone())
        .then(item.clone())
        .then_ignore(just(Token::WithHandler))
        .then(
            ty.clone()
                .then_ignore(just(Token::EArrow))
                .then(item.clone())
                .separated_by(just(Token::Comma)),
        )
        .map(|((class, item), handlers)| (class, item, handlers))
        .dot()
}

pub(crate) fn parse_op<U, O>(
    op: impl Parser<Token, U, Error = Simple<Token>> + Clone,
    item: impl Parser<Token, Spanned<O>, Error = Simple<Token>> + Clone,
) -> impl Parser<Token, Vec<Spanned<O>>, Error = Simple<Token>> + Clone {
    op.ignore_then(item.separated_by(just(Token::Comma))).dot()
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

pub(crate) trait ParserExt<O>
where
    Self: Parser<Token, O> + Sized,
{
    fn dot(
        self,
    ) -> Map<
        Then<Self, OrNot<Just<Token, Self::Error>>>,
        fn((O, Option<Token>)) -> O,
        (O, Option<Token>),
    >;
}

impl<T: Parser<Token, O, Error = E>, O, E: Error<Token>> ParserExt<O> for T {
    fn dot(
        self,
    ) -> Map<
        Then<Self, OrNot<Just<Token, Self::Error>>>,
        fn((O, Option<Token>)) -> O,
        (O, Option<Token>),
    >
    where
        Self: Sized,
    {
        self.then_ignore(just(Token::Dot).or_not())
    }
}
