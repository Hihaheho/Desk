use crate::textual_diagnostics::TextualDiagnostics;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SyntaxError {
	#[error("other error {0}")]
	Other(String),
}

impl From<&SyntaxError> for TextualDiagnostics {
    fn from(value: &SyntaxError) -> Self {
		match value {
			SyntaxError::Other(string) => TextualDiagnostics { title: string.clone(), reports: vec![] },
		}
    }
}
