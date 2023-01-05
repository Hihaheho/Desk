use dson::Dson;
use ids::NodeId;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Meta {
    pub id: NodeId,
    pub attrs: Vec<Dson>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct WithMeta<T: std::fmt::Debug> {
    pub meta: Meta,
    pub value: T,
}

// This is intended to be used in tests.
pub fn dummy_meta<T: std::fmt::Debug>(value: T) -> WithMeta<T> {
    WithMeta {
        meta: Meta::default(),
        value,
    }
}
