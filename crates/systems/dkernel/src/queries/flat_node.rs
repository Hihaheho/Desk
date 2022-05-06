use std::sync::Arc;

use dkernel_card::{flat_node::FlatNode, node::NodeId};

use super::KernelQueries;

pub(super) fn flat_node(db: &dyn KernelQueries, id: NodeId) -> Arc<FlatNode> {
    todo!()
}
