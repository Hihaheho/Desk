use ids::NodeId;
use uuid::Uuid;

pub type Span = std::ops::Range<usize>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WithSpan<T> {
    pub id: NodeId,
    pub span: Span,
    pub value: T,
}

pub fn dummy_span<T>(value: T) -> WithSpan<T> {
    WithSpan {
        id: NodeId(Uuid::default()),
        span: 0..0,
        value,
    }
}
