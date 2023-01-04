use ids::NodeId;

pub type Span = std::ops::Range<usize>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WithSpan<T> {
    pub id: NodeId,
    pub span: Span,
    pub value: T,
}

/// This trait should be implemented by items that can provide span information
pub trait ToSpan {
    /// Calculates the span of the implementing item, ideally by passing it through to
    /// the generated `parol_runtime::ToSpan::span`.
    fn span(&self) -> Span;
}

pub fn dummy_span<T>(value: T) -> WithSpan<T> {
    WithSpan {
        id: NodeId::default(),
        span: 0..0,
        value,
    }
}

/// Equips a value with its span information
pub fn with_span<T>(id: NodeId, value: T) -> WithSpan<T>
where
    T: ToSpan,
{
    WithSpan {
        id,
        span: value.span(),
        value,
    }
}

/// Equips a value with span information from a span provider
pub fn with_span_from<S, T>(id: NodeId, value: T, span_provider: &S) -> WithSpan<T>
where
    S: ToSpan,
{
    WithSpan {
        id,
        span: span_provider.span(),
        value,
    }
}
