use deskc_ids::LinkName;
use types::Type;

#[derive(Clone, Debug, PartialEq)]
pub enum Content {
    SourceCode { syntax: SyntaxKind, source: String },
    String(String),
    Integer(i64),
    Rational(i64, i64),
    Float(f64),
    Apply { ty: Type, link_name: LinkName },
}

// Content::Float should not be NaN
impl Eq for Content {}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ContentKind {
    SourceCode,
    String,
    Integer,
    Rational,
    Float,
    Apply,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SyntaxKind {
    Hacker,
    TypeScriptLike,
    OCamlLike,
    RustLike,
}

impl Content {
    pub fn kind(&self) -> ContentKind {
        match self {
            Content::SourceCode { .. } => ContentKind::SourceCode,
            Content::String(_) => ContentKind::String,
            Content::Integer(_) => ContentKind::Integer,
            Content::Rational(_, _) => ContentKind::Rational,
            Content::Float(_) => ContentKind::Float,
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
                syntax: SyntaxKind::Hacker,
                source: String::new()
            }
            .kind(),
            ContentKind::SourceCode
        );
        assert_eq!(Content::String(String::new()).kind(), ContentKind::String);
        assert_eq!(Content::Integer(0).kind(), ContentKind::Integer);
        assert_eq!(Content::Rational(0, 1).kind(), ContentKind::Rational);
        assert_eq!(Content::Float(0.0).kind(), ContentKind::Float);
        assert_eq!(
            Content::Apply {
                ty: Type::Number,
                link_name: LinkName::None
            }
            .kind(),
            ContentKind::Apply
        );
    }
}
