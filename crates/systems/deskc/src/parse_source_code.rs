use ast::{expr::Expr, span::WithSpan};
use codebase::code::SyntaxKind;

pub fn parse_source_code(
    syntax: &SyntaxKind,
    source: &str,
) -> Result<WithSpan<Expr>, anyhow::Error> {
    match syntax {
        SyntaxKind::Minimalist => Ok(minimalist::parse(source)?),
        SyntaxKind::TypeScriptLike => todo!(),
        SyntaxKind::OCamlLike => todo!(),
        SyntaxKind::RustLike => todo!(),
    }
}
