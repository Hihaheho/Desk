use std::sync::Arc;

use components::node::Node;
use deskc_ids::NodeId;

use super::HirQueries;

pub(super) fn ast(db: &dyn HirQueries, id: NodeId) -> Arc<Node> {
    let flat_node = db.flat_node(id.clone());
    Arc::new(Node {
        id,
        file_id: flat_node.file_id.clone(),
        content: flat_node.content.clone(),
        children: flat_node
            .children
            .iter()
            .map(|child_id| db.ast(child_id.clone()).as_ref().clone())
            .collect(),
        attributes: flat_node.attributes.clone(),
    })
}
