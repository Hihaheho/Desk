use ids::NodeId;

pub type Span = std::ops::Range<usize>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WithSpan<T> {
    pub id: NodeId,
    pub span: Span,
    pub value: T,
}

/// This trait should be implemented by generated AST data types
pub trait ToSpan {
    /// Calculates the span of the implementing item
    fn span(&self) -> Span;
}


pub fn dummy_span<T>(value: T) -> WithSpan<T> {
    WithSpan {
        id: NodeId::default(),
        span: 0..0,
        value,
    }
}

pub fn with_span<T>(value: T) -> WithSpan<T>  where T: ToSpan {
    WithSpan {
        id: NodeId::default(),
        span: value.span(),
        value,
    }
}
