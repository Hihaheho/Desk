use language::code::node::Node;
use protocol::card_id::CardId;

pub struct Card {
    pub id: CardId,
}

impl Card {
    pub fn new() -> Self {
        Self { id: CardId::new() }
    }
}

/// A struct for a computed value with its type and encoding.
pub struct Computed(pub Node);
