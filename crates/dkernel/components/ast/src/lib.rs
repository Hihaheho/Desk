pub mod content;
pub mod flat_node;
pub mod patch;

use std::collections::HashMap;

use content::Content;
use hir::expr::Expr;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NodeId(pub Uuid);

pub struct Node {
    pub content: Content,
    pub children: Vec<Node>,
    pub attributes: HashMap<AttributeKey, Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttributeKey {
    pub crate_name: String,
    pub key: String,
}
