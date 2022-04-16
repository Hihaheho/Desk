use ast::span::Span;
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum HirGenError {
    #[error("class expected")]
    ClassExpected { span: Span },
    #[error("unexpected class")]
    UnexpectedClass { span: Span },
    #[error("unknown type alias {alias} {span:?}")]
    UnknownTypeAlias { alias: String, span: Span },
}
