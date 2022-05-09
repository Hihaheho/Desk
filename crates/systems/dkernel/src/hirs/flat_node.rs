use std::sync::Arc;

use dkernel_card::{flat_node::FlatNode, node::NodeId};

use super::HirQueries;

pub(super) fn flat_node(db: &dyn HirQueries, id: NodeId) -> Arc<FlatNode> {
    Arc::new(FlatNode {
        content: db.content(id.clone()),
        children: db.children(id.clone()),
        attributes: db.attributes(id),
    })
}
