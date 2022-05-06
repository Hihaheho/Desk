use std::collections::HashMap;

use hir::expr::Expr;
use uuid::Uuid;

use crate::{content::Content, AttributeKey};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NodeId(pub Uuid);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node {
    pub content: Content,
    pub children: Vec<Node>,
    pub attributes: HashMap<AttributeKey, Expr>,
}
