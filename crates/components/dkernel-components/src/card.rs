use deskc_ids::{FileId, NodeId};

// TODO: delete this?
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Card {
    node_id: NodeId,
    file_id: FileId,
}
