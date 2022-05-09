use std::collections::HashMap;

use hir::expr::Expr;
use types::Type;

use crate::{content::Content, node::NodeId};

pub type Children = Vec<NodeRef>;
pub type Attributes = HashMap<Type, Expr>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlatNode {
    /// The content of the node.
    pub content: Content,
    pub children: Children,
    pub attributes: Attributes,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeRef {
    Hole,
    Node(NodeId),
}
