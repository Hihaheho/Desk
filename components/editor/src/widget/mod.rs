use language::abstract_syntax_tree::{node::NumberLiteral, path::NodePath};
use protocol::card_id::CardId;

#[derive(Clone, Debug)]
pub struct Target {
    pub card_id: CardId,
    pub node_path: NodePath,
}

#[derive(Clone, Debug)]
pub enum Widget {
    Unit,
    InputString {
        value: String,
        target: Target,
    },
    InputNumber {
        value: NumberLiteral,
        target: Target,
    },
}
