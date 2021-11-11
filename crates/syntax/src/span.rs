pub type Span = std::ops::Range<usize>;

pub type Spanned<T> = (T, Span);
