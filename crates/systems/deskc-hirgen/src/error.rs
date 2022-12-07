use ids::{CardId, NodeId};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum HirGenError {
    #[error("class expected")]
    ClassExpected { span: NodeId },
    #[error("unexpected class")]
    UnexpectedClass { span: NodeId },
    #[error("unknown type alias {alias} {span:?}")]
    UnknownTypeAlias { alias: String, span: NodeId },
    #[error("unexpected card {card_id:?}")]
    UnexpectedCard { card_id: CardId },
}
