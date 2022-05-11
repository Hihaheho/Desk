use deskc_ids::{FileId, NodeId};

use crate::{content::Content, flat_node::Attributes};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node {
    pub id: NodeId,
    pub file_id: FileId,
    pub content: Content,
    pub children: Vec<Node>,
    pub attributes: Attributes,
}
