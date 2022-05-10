use deskc_ids::NodeId;
use dkernel_card::{content::Content, node::Node};

use crate::query_result::QueryResult;

use super::HirQueries;

pub(super) fn build_ast(db: &dyn HirQueries, id: NodeId) -> QueryResult<Node> {
    if let Content::Source(_source) = db.content(id) {
    } else {
    }
    todo!()
}
