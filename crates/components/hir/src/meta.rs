use std::ops::Range;

pub type Id = usize;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Meta {
    pub id: Id,
    pub span: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WithMeta<T> {
    pub meta: Option<Meta>,
    pub value: T,
}

pub fn no_meta<T>(value: T) -> WithMeta<T> {
    WithMeta {
        meta: None,
        value,
    }
}
