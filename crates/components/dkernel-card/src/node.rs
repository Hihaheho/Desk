use uuid::Uuid;

use crate::{
    content::Content,
    flat_node::{Attributes, Children},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NodeId(pub Uuid);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node {
    pub content: Content,
    pub children: Children,
    pub attributes: Attributes,
}
