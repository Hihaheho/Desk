pub mod content;
pub mod flat_node;
pub mod node;
pub mod patch;
pub mod file;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttributeKey {
    pub crate_name: String,
    pub key: String,
}
