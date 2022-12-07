use deskc_ids::LinkName;
use types::Type;

use crate::code::SyntaxKind;

#[derive(Clone, Debug, PartialEq)]
pub enum Content {
    SourceCode { syntax: SyntaxKind, source: String },
    String(String),
    Integer(i64),
    // b must be unsigned to avoid ambiguity.
    Rational(i64, u64),
    Real(f64),
    Apply { ty: Type, link_name: LinkName },
}

// Content::Real should not be NaN
impl Eq for Content {}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ContentKind {
    SourceCode,
    String,
    Integer,
    Rational,
    Real,
    Apply,
}

impl Content {
    pub fn kind(&self) -> ContentKind {
        match self {
            Content::SourceCode { .. } => ContentKind::SourceCode,
            Content::String(_) => ContentKind::String,
            Content::Integer(_) => ContentKind::Integer,
            Content::Rational(_, _) => ContentKind::Rational,
            Content::Real(_) => ContentKind::Real,
            Content::Apply { .. } => ContentKind::Apply,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_kind() {
        assert_eq!(
            Content::SourceCode {
                syntax: SyntaxKind::Minimalist,
                source: String::new()
            }
            .kind(),
            ContentKind::SourceCode
        );
        assert_eq!(Content::String(String::new()).kind(), ContentKind::String);
        assert_eq!(Content::Integer(0).kind(), ContentKind::Integer);
        assert_eq!(Content::Rational(0, 1).kind(), ContentKind::Rational);
        assert_eq!(Content::Real(0.0).kind(), ContentKind::Real);
        assert_eq!(
            Content::Apply {
                ty: Type::Real,
                link_name: LinkName::None
            }
            .kind(),
            ContentKind::Apply
        );
    }
}
