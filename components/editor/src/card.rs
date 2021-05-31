use language::abstract_syntax_tree::node::Node;
use protocol::card_id::CardId;

pub struct Card {
    pub card_id: CardId,
}

/// A struct for a computed value with its type and encoding.
pub struct Computed(pub Node);
