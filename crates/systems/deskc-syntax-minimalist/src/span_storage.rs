use ast::{
    meta::Span,
    parser::{dyn_eq, SpanStorage},
};

#[derive(Debug, PartialEq, Eq)]
pub struct MinimalistSyntaxSpanStorage {}

impl SpanStorage for MinimalistSyntaxSpanStorage {
    fn calculate_span(&self, _id: &ids::NodeId) -> Option<Span> {
        None
    }
    fn dyn_eq(&self, other: &dyn SpanStorage) -> bool {
        dyn_eq(self, other)
    }
}
