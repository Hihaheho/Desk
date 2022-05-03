use types::Type;
use uuid::Uuid;

pub struct Node {
    pub content: Content,
    pub children: Vec<NodeRef>,
}

pub enum Content {
    String(String),
    Integer(u64),
    Float(f64),
    Type(Type),
    Apply,
}

pub enum NodeRef {
    Hole,
    Node(Uuid),
}
