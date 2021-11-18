use std::ops::Range;

use crate::expr::Expr;

pub type Id = usize;

#[derive(Clone, Debug, PartialEq)]
pub struct Meta {
    pub attr: Option<Box<Expr>>,
    pub id: Id,
    pub span: Range<usize>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WithMeta<T> {
    pub meta: Option<Meta>,
    pub value: T,
}

pub fn no_meta<T>(value: T) -> WithMeta<T> {
    WithMeta { meta: None, value }
}
