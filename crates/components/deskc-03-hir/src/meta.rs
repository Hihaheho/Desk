use std::ops::Range;

use ids::IrId;
use uuid::Uuid;

use crate::expr::Expr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Meta {
    pub attrs: Vec<Expr>,
    pub id: IrId,
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
            id: IrId(Uuid::default()),
            span: 0..0,
        },
        value,
    }
}
