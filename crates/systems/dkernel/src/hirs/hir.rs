use std::{error::Error, sync::Arc};

use components::{content::Content, node::Node};
use deskc_hir::{
    expr::{Expr, Literal},
    meta::{Meta, WithMeta},
};
use deskc_hirgen::gen_hir;
use deskc_ids::{CardId, IrId};
use deskc_lexer::scan;
use deskc_parser::parse;

use crate::{
    error::KernelError,
    query_result::{QueryError, QueryResult},
};

use super::HirQueries;

pub(super) fn hir(db: &dyn HirQueries, id: CardId) -> QueryResult<WithMeta<Expr>> {
    let node_id = db.node_id(id);
    let ast = db.build_ast(node_id)?;

    Ok(Arc::new(genhir(&*ast)?))
}

fn genhir(node: &Node) -> Result<WithMeta<Expr>, QueryError> {
    let meta = Meta {
        attrs: vec![],
        file_id: node.file_id.clone(),
        node_id: Some(node.id.clone()),
        span: None,
    };
    let expr = match &node.content {
        Content::Source(source) => {
            let tokens = scan(source)?;
            let ast = parse(tokens)?;
            let (_, hir) = gen_hir(&ast)?;
            return Ok(hir.entrypoint.ok_or(KernelError::NoEntrypoint {
                node_id: node.id.clone(),
            })?);
        }
        Content::String(string) => Expr::Literal(Literal::String(string.clone())),
        Content::Integer(integer) => Expr::Literal(Literal::Integer(integer.clone())),
        Content::Rational(a, b) => Expr::Literal(Literal::Rational(a.clone(), b.clone())),
        Content::Float(float) => Expr::Literal(Literal::Float(float.clone())),
        Content::Apply { ty, link_name } => Expr::Apply {
            function: ty.clone(),
            link_name: link_name.clone(),
            arguments: node
                .children
                .iter()
                .map(|child| genhir(child))
                .collect::<Result<Vec<_>, _>>()?,
        },
    };
    Ok(WithMeta {
        id: IrId::new(),
        meta: meta,
        value: expr,
    })
}
