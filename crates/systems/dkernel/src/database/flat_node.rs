use std::sync::Arc;

use dkernel_card::{flat_node::FlatNode, node::NodeId};

use super::Queries;

pub(super) fn flat_node(db: &dyn Queries, id: NodeId) -> Arc<FlatNode> {
    todo!()
}
