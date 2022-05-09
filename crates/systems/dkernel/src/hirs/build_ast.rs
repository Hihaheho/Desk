use deskc_ast::{expr::Expr, span::Spanned};
use dkernel_card::{
    content::Content,
    node::{Node, NodeId},
};

use crate::query_result::QueryResult;

use super::HirQueries;

pub(super) fn build_ast(db: &dyn HirQueries, id: NodeId) -> QueryResult<Node> {
    if let Content::Source(source) = db.content(id.clone()) {
    } else {
    }
    todo!()
}
