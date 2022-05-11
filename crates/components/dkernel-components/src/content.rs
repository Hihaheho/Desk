use deskc_ids::LinkName;
use types::Type;

#[derive(Clone, Debug, PartialEq)]
pub enum Content {
    Source(String),
    String(String),
    Integer(i64),
    Rational(i64, i64),
    Float(f64),
    Apply { ty: Type, link_name: LinkName },
}

// Content::Float should not be NaN
impl Eq for Content {}
