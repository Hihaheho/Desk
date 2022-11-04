use deskc_ids::NodeId;

use crate::{content::Content, flat_node::Attributes};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node {
    pub id: NodeId,
    pub content: Content,
    pub operands: Vec<Node>,
    pub attributes: Attributes,
}
