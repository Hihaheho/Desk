use std::sync::Arc;

use components::flat_node::FlatNode;
use deskc_ids::NodeId;

use super::HirQueries;

pub(super) fn flat_node(db: &dyn HirQueries, id: NodeId) -> Arc<FlatNode> {
    Arc::new(FlatNode {
        file_id: db.file_id(id.clone()),
        content: db.content(id.clone()),
        children: db.children(id.clone()),
        attributes: db.attributes(id),
    })
}
