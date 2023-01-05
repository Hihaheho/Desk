use ids::NodeId;

pub type Span = std::ops::Range<usize>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WithMeta<T> {
    pub meta: Meta,
    pub value: T,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Meta {
    pub id: NodeId,
    pub comments: Vec<Comment>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Comment {
    Line(String),
    Block(String),
}

// This is intended to be used in tests.
pub fn dummy_meta<T>(value: T) -> WithMeta<T> {
    WithMeta {
        meta: Meta::default(),
        value,
    }
}

impl Meta {
    pub fn new_no_comments() -> Self {
        Self {
            id: NodeId::new(),
            comments: vec![],
        }
    }
}

impl From<NodeId> for Meta {
    fn from(id: NodeId) -> Self {
        Self {
            id,
            comments: vec![],
        }
    }
}
