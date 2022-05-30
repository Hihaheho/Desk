use std::sync::Arc;

use components::node::Node;
use deskc_ids::NodeId;

use super::NodeQueries;

pub(super) fn node(db: &dyn NodeQueries, id: NodeId) -> Arc<Node> {
    let flat_node = db.flat_node(id.clone());
    Arc::new(Node {
        id,
        file_id: flat_node.file_id.clone(),
        content: flat_node.content.clone(),
        children: flat_node
            .children
            .iter()
            .map(|child_id| db.node(child_id.clone()).as_ref().clone())
            .collect(),
        attributes: flat_node.attributes.clone(),
    })
}
