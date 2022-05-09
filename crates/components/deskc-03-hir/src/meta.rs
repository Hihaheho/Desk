use std::ops::Range;

use crate::expr::Expr;

pub type Id = usize;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Meta {
    pub attrs: Vec<Expr>,
    pub id: Id,
    pub span: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WithMeta<T> {
    pub meta: Meta,
    pub value: T,
}

pub fn dummy_meta<T>(value: T) -> WithMeta<T> {
    WithMeta {
        meta: Meta {
            attrs: vec![],
            id: 0,
            span: 0..0,
        },
        value,
    }
}
