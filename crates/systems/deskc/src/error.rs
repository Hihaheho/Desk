use ids::{CardId, FileId};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeskcError {
    #[error("card not found: {card_id:?} in {file_id:?}")]
    CardNotFound { card_id: CardId, file_id: FileId },
}
