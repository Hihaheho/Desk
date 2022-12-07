use std::ops::Range;

use dson::Dson;
use ids::NodeId;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Meta {
    pub id: NodeId,
    pub attrs: Vec<Dson>,
    pub span: Option<Range<usize>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WithMeta<T> {
    pub meta: Meta,
    pub value: T,
}

pub fn dummy_meta<T>(value: T) -> WithMeta<T> {
    WithMeta {
        meta: Meta::default(),
        value,
    }
}
