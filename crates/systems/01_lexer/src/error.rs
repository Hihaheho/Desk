use chumsky::prelude::Simple;
use textual_diagnostics::{Report, TextualDiagnostics};

#[derive(Debug, PartialEq)]
pub struct LexerError(pub Vec<Simple<char>>);

impl Into<TextualDiagnostics> for LexerError {
    fn into(self) -> TextualDiagnostics {
        TextualDiagnostics {
            title: "Lexer error".into(),
            reports: self
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
