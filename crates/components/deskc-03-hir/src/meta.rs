use std::ops::Range;

use dson::Dson;
use ids::NodeId;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Meta {
    pub attrs: Vec<Dson>,
    pub span: Option<Range<usize>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WithMeta<T> {
    pub id: NodeId,
    pub meta: Meta,
    pub value: T,
}

pub fn dummy_meta<T>(value: T) -> WithMeta<T> {
    WithMeta {
        id: NodeId(Uuid::default()),
        meta: Meta::default(),
        value,
    }
}
