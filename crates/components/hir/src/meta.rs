use std::ops::Range;

use file::FileId;

use crate::expr::Expr;

pub type Id = usize;

#[derive(Clone, Debug, PartialEq)]
pub struct Meta {
    pub attrs: Vec<Expr>,
    pub id: Id,
    pub file_id: FileId,
    pub span: Range<usize>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WithMeta<T> {
    pub meta: Meta,
    pub value: T,
}

pub fn dummy_meta<T>(value: T) -> WithMeta<T> {
    WithMeta {
        meta: Meta {
            attrs: vec![],
            id: 0,
            file_id: FileId(0),
            span: 0..0,
        },
        value,
    }
}
