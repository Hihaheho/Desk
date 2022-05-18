use ids::NodeId;
use thiserror::Error;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum HirGenError {
    #[error("class expected")]
    ClassExpected { span: NodeId },
    #[error("unexpected class")]
    UnexpectedClass { span: NodeId },
    #[error("unknown type alias {alias} {span:?}")]
    UnknownTypeAlias { alias: String, span: NodeId },
    #[error("unexpected card {ident}")]
    UnexpectedCard { ident: Uuid },
}
