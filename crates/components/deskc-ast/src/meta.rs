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
    pub comments: Comments,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Comment {
    Line(String),
    Block(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Comments {
    pub before: Vec<Comment>,
    pub after: Option<String>,
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
            comments: Default::default(),
        }
    }
}

impl From<NodeId> for Meta {
    fn from(id: NodeId) -> Self {
        Self {
            id,
            comments: Default::default(),
        }
    }
}

impl From<Vec<Comment>> for Comments {
    fn from(comments: Vec<Comment>) -> Self {
        Self {
            before: comments,
            after: None,
        }
    }
}
