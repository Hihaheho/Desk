use deskc_hir::{
    expr::{Expr, Literal},
    meta::{Meta, WithMeta},
};
use deskc_ids::{CardId, IrId};
use dkernel_card::{content::Content, node::Node};

use crate::query_result::{QueryError, QueryResult};

use super::HirQueries;

pub(super) fn hir(db: &dyn HirQueries, id: CardId) -> QueryResult<WithMeta<Expr>> {
    let node_id = db.node_id(id);
    let ast = db.build_ast(node_id)?;

    let _ = genhir(&*ast);

    todo!()
}

fn genhir(ast: &Node) -> Result<WithMeta<Expr>, QueryError> {
    let expr = match &ast.content {
        Content::Source(_source) => todo!(),
        Content::String(string) => WithMeta {
            id: IrId::new(),
            // TODO: node_id
            meta: Meta::default(),
            value: Expr::Literal(Literal::String(string.clone())),
        },
        Content::Integer(_integer) => todo!(),
        Content::Rational(_a, _b) => todo!(),
        Content::Float(_float) => todo!(),
        Content::Apply(_ty) => todo!(),
    };
    Ok(expr)
}
