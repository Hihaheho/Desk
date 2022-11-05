use crate::{
    content::Content,
    patch::{AttributePatch, ContentPatch, OperandsPatch},
    rules::{NodeOperation, Rules, SpaceOperation},
    user::UserId,
};
use deskc_ids::NodeId;

use crate::snapshot::Snapshot;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventEntry {
    pub index: usize,
    pub user_id: UserId,
    pub event: Event,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    AddOwner {
        user_id: UserId,
    },
    RemoveOwner {
        user_id: UserId,
    },
    UpdateSpaceRules {
        rules: Rules<SpaceOperation>,
    },
    AddNode {
        parent: Option<NodeId>,
        node_id: NodeId,
        content: Content,
    },
    RemoveNode {
        node_id: NodeId,
    },
    UpdateParent {
        node_id: NodeId,
        parent: Option<NodeId>,
    },
    PatchContent {
        node_id: NodeId,
        patch: ContentPatch,
    },
    PatchOperands {
        node_id: NodeId,
        patch: OperandsPatch,
    },
    PatchAttribute {
        node_id: NodeId,
        patch: AttributePatch,
    },
    UpdateNodeRules {
        node_id: NodeId,
        rules: Rules<NodeOperation>,
    },
    AddSnapshot {
        /// current index of events
        /// if the index of this event is not index+1, this event must be
        index: usize,
        snapshot: Box<Snapshot>,
    },
}
