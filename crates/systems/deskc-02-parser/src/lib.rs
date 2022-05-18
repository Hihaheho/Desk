use ast::{
    expr::Expr,
    span::{Span, WithSpan},
};
use chumsky::prelude::Simple;
use textual_diagnostics::{Report, TextualDiagnostics};
use thiserror::Error;
use tokens::Token;

pub mod common;
pub mod expr;
pub mod ty;

pub fn parse(input: Vec<(Token, Span)>) -> Result<WithSpan<Expr>, ParserError> {
    use chumsky::prelude::*;
    use chumsky::Stream;
    expr::parser()
        .then_ignore(end())
        .parse(Stream::from_iter(
            input.len()..input.len() + 1,
            input.into_iter(),
        ))
        .map_err(ParserError)
}

#[derive(Error, Debug, PartialEq)]
#[error("{0:?}")]
pub struct ParserError(pub Vec<Simple<Token>>);

impl From<ParserError> for TextualDiagnostics {
    fn from(error: ParserError) -> TextualDiagnostics {
        TextualDiagnostics {
            title: "Parser error".into(),
            reports: error
                .0
                .into_iter()
                .map(|error| Report {
                    span: error.span(),
                    text: format!("{:?}", error),
                })
                .collect(),
        }
    }
}
