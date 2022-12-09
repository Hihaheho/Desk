use std::ops::Range;

use dson::Dson;
use ids::NodeId;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Meta {
    pub id: NodeId,
    pub attrs: Vec<Dson>,
    pub span: Option<Range<usize>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct WithMeta<T: std::fmt::Debug> {
    pub meta: Meta,
    pub value: T,
}

pub fn dummy_meta<T: std::fmt::Debug>(value: T) -> WithMeta<T> {
    WithMeta {
        meta: Meta::default(),
        value,
    }
}
