use chumsky::prelude::Simple;
use textual_diagnostics::{Report, TextualDiagnostics};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
#[error("{0:?}")]
pub struct LexerError(pub Vec<Simple<char>>);

impl From<LexerError> for TextualDiagnostics {
    fn from(error: LexerError) -> TextualDiagnostics {
        TextualDiagnostics {
            title: "Lexer error".into(),
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
