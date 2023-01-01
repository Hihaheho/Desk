use ids::NodeId;

pub type Span = std::ops::Range<usize>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WithSpan<T> {
    pub id: NodeId,
    pub span: Span,
    pub value: T,
}

pub fn with_span<T>(value: T) -> WithSpan<T>  where T: parol_runtime::ToSpan {
    WithSpan {
        id: NodeId::default(),
        span: (&value).span().into(),
        value,
    }
}
