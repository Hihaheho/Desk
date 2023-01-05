use ast::{expr::Expr, meta::WithMeta};
use codebase::code::SyntaxKind;

pub fn parse_source_code(
    syntax: &SyntaxKind,
    source: &str,
) -> Result<WithMeta<Expr>, anyhow::Error> {
    match syntax {
        SyntaxKind::Minimalist => Ok(minimalist::parse(source)?),
        SyntaxKind::TypeScriptLike => todo!(),
        SyntaxKind::OCamlLike => todo!(),
        SyntaxKind::RustLike => todo!(),
    }
}
