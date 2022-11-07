use components::{
    content::ContentKind,
    rules::{NodeOperation, SpaceOperation},
};
use deskc_ids::NodeId;

#[derive(Debug, PartialEq)]
pub enum Assertion<'a> {
    SpaceAllows(SpaceOperation),
    NodeAllows {
        node_id: &'a NodeId,
        operation: NodeOperation,
    },
    Owner,
    NoOwner,
    NodeExists(&'a NodeId),
    NotReferenced(&'a NodeId),
    NoOperandLoop {
        node_id: &'a NodeId,
        operand_id: &'a NodeId,
    },
    OperandsHasSize {
        node_id: &'a NodeId,
        size: usize,
    },
    ContentKind {
        node_id: &'a NodeId,
        kind: ContentKind,
    },
    All(Vec<Assertion<'a>>),
    Any(Vec<Assertion<'a>>),
}
