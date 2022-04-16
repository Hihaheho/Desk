use ast::{expr::Expr, span::Span};
use chumsky::prelude::Simple;
use tokens::Token;

pub mod common;
pub mod expr;
pub mod ty;

pub fn parse(input: Vec<(Token, Span)>) -> Result<(Expr, Span), Vec<Simple<Token>>> {
    use chumsky::prelude::*;
    use chumsky::Stream;
    expr::parser().then_ignore(end()).parse(Stream::from_iter(
        input.len()..input.len() + 1,
        input.into_iter(),
    ))
}
