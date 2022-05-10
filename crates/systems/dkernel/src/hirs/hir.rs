use deskc_hir::{expr::Expr, meta::WithMeta};
use deskc_ids::CardId;
use dkernel_card::{content::Content, node::Node};

use crate::query_result::{QueryError, QueryResult};

use super::HirQueries;

pub(super) fn hir(db: &dyn HirQueries, id: CardId) -> QueryResult<WithMeta<Expr>> {
    let node_id = db.node_id(id);
    let ast = db.build_ast(node_id)?;

    genhir(&*ast);

    todo!()
}

fn genhir(ast: &Node) -> Result<WithMeta<Expr>, QueryError> {
    let expr = match &ast.content {
        Content::Source(source) => todo!(),
        Content::String(string) => WithMeta {
            id: todo!(),
            meta: todo!(),
            value: todo!(),
        },
        Content::Integer(integer) => todo!(),
        Content::Rational(a, b) => todo!(),
        Content::Float(float) => todo!(),
        Content::Apply(ty) => todo!(),
    };
    Ok(expr)
}
