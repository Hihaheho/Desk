use components::{
    content::ContentKind,
    rules::{NodeOperation, SpaceOperation},
};
use deskc_ids::NodeId;

use super::execute_assertion::AssertionError;

#[derive(Debug, PartialEq)]
pub enum Assertion {
    SpaceAllows(SpaceOperation),
    NodeAllows {
        node_id: NodeId,
        operation: NodeOperation,
    },
    Owner,
    NoOwner,
    NodeExists(NodeId),
    NotReferenced(NodeId),
    NoOperandLoop {
        node_id: NodeId,
        operand_id: NodeId,
    },
    OperandsHasSize {
        node_id: NodeId,
        size: usize,
    },
    ContentKind {
        node_id: NodeId,
        kind: ContentKind,
    },
    HasOperand {
        node_id: NodeId,
        operand_id: NodeId,
    },
    All(Vec<Assertion>),
    Any(Vec<Assertion>),
    Contradiction(AssertionError),
}

impl Assertion {
    pub fn tautology() -> Self {
        Self::All(Vec::new())
    }
}
