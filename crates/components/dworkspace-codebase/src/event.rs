use crate::{
    content::Content,
    patch::{AttributePatch, ContentPatch, OperandPatch},
    rules::{NodeOperation, Rules, SpaceOperation},
    user::UserId,
};
use deskc_ids::NodeId;
use uuid::Uuid;

use crate::projection::Projection;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EventId(pub Uuid);

impl EventId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: EventId,
    pub user_id: UserId,
    pub payload: EventPayload,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventPayload {
    AddOwner {
        user_id: UserId,
    },
    RemoveOwner {
        user_id: UserId,
    },
    UpdateSpaceRules {
        rules: Rules<SpaceOperation>,
    },
    CreateNode {
        node_id: NodeId,
        content: Content,
    },
    RemoveNode {
        node_id: NodeId,
    },
    PatchContent {
        node_id: NodeId,
        patch: ContentPatch,
    },
    PatchOperand {
        node_id: NodeId,
        patch: OperandPatch,
    },
    PatchAttribute {
        node_id: NodeId,
        patch: AttributePatch,
    },
    UpdateNodeRules {
        node_id: NodeId,
        rules: Rules<NodeOperation>,
    },
    UpdateOperandRules {
        node_id: NodeId,
        rules: Rules<NodeOperation>,
    },
    AddSnapshot {
        /// current index of events
        /// if the index of this event is not index+1, this event must be
        index: usize,
        snapshot: Box<Projection>,
    },
}
