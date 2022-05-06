use std::collections::HashMap;

use hir::expr::Expr;
use uuid::Uuid;

use crate::{content::Content, AttributeKey};

pub type Attributes = HashMap<AttributeKey, Expr>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlatNode {
    /// The content of the node.
    pub content: Content,
    pub children: Vec<NodeRef>,
    pub attributes: Attributes,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeRef {
    Hole,
    Node(Uuid),
}
