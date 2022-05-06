pub mod card;
pub mod content;
pub mod file;
pub mod flat_node;
pub mod node;
pub mod patch;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttributeKey {
    pub crate_name: String,
    pub key: String,
}
