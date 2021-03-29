use crate::Source;

pub type CardId = String;

/// Has all data that picked up from cards universe.
/// Agnostic on:
/// - Front or backend
/// - Data store
/// - Computation
/// - Actual data
pub struct Card {
    pub id: CardId,
    pub source: Box<dyn Source>,
}

impl Card {
    pub fn new<S: Source + 'static>(source: S) -> Self {
        let id = uuid::Uuid::new_v4().as_u128().to_string();
        Self {
            id,
            source: Box::new(source),
        }
    }
}
