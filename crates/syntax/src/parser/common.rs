use chumsky::prelude::*;

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
}
