use std::sync::Arc;

use downcast_rs::{impl_downcast, DowncastSync};
use ids::NodeId;

use crate::{
    expr::Expr,
    meta::{Span, WithMeta},
};

pub trait Parser {
    type Error;

    fn parse(input: &str) -> Result<ParseResult, Self::Error>;
}

#[derive(Debug, Eq, Clone)]
pub struct ParseResult {
    pub expr: Arc<WithMeta<Expr>>,
    pub span_storage: Arc<Box<dyn SpanStorage>>,
}

impl PartialEq for ParseResult {
    fn eq(&self, other: &Self) -> bool {
        self.expr == other.expr && self.span_storage == other.span_storage
    }
}

impl ParseResult {
    pub fn new<T: SpanStorage + 'static>(expr: Arc<WithMeta<Expr>>, span_storage: T) -> Self {
        Self {
            expr,
            span_storage: Arc::new(Box::new(span_storage)),
        }
    }
}

pub trait SpanStorage: std::fmt::Debug + DowncastSync + Sync + Send {
    fn calculate_span(&self, id: &NodeId) -> Option<Span>;
    /// Use `deskc_ast::parser::dyn_eq` for implementation.
    fn dyn_eq(&self, other: &dyn SpanStorage) -> bool;
}

impl_downcast!(sync SpanStorage);

impl PartialEq for dyn SpanStorage {
    fn eq(&self, other: &Self) -> bool {
        self.dyn_eq(other)
    }
}

impl Eq for dyn SpanStorage {}

#[derive(Debug, PartialEq, Eq)]
pub struct DummySpanStorage;
impl SpanStorage for DummySpanStorage {
    fn calculate_span(&self, _id: &ids::NodeId) -> Option<Span> {
        None
    }
    fn dyn_eq(&self, other: &dyn SpanStorage) -> bool {
        dyn_eq(self, other)
    }
}

pub fn dyn_eq<T: SpanStorage + PartialEq>(a: &T, b: &dyn SpanStorage) -> bool {
    b.downcast_ref::<T>().map_or(false, |b| a == b)
}
