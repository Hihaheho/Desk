use types::Type;

#[derive(Clone, Debug, PartialEq)]
pub enum Content {
    String(String),
    Integer(u64),
    Rational(u64, u64),
    Float(f64),
    Apply(Type),
}

// Content::Float should not be NaN
impl Eq for Content {}
