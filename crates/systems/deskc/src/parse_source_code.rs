use ast::{expr::Expr, span::WithSpan};
use codebase::code::SyntaxKind;

pub fn parse_source_code(
    _syntax: &SyntaxKind,
    source: &str,
) -> Result<WithSpan<Expr>, anyhow::Error> {
    let tokens = lexer::scan(source)?;
    let ast = parser::parse(tokens)?;
    Ok(ast)
}
