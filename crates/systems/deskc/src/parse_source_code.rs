use ast::parser::{ParseResult, Parser};
use codebase::code::SyntaxKind;

pub fn parse_source_code(syntax: &SyntaxKind, source: &str) -> Result<ParseResult, anyhow::Error> {
    match syntax {
        SyntaxKind::Minimalist => Ok(minimalist::MinimalistSyntaxParser::parse(source)?),
    }
}
