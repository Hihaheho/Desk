use std::sync::Arc;

use components::node::Node;
use deskc_ids::NodeId;

use super::NodeQueries;

pub(super) fn node(db: &dyn NodeQueries, id: NodeId) -> Arc<Node> {
    let flat_node = db.flat_node(id.clone());
    Arc::new(Node {
        id,
        content: flat_node.content.clone(),
        operands: flat_node
            .operands
            .iter()
            .map(|child_id| db.node(child_id.clone()).as_ref().clone())
            .collect(),
        attributes: flat_node.attributes.clone(),
    })
}
