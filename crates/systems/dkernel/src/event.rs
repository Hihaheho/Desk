use deskc_ids::{CardId, FileId, UserId};
use dkernel_card::{
    content::Content,
    node::NodeId,
    patch::{AttributePatch, ChildrenPatch, ContentPatch},
};

use crate::snapshot::Snapshot;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventEntry {
    pub index: usize,
    pub user_id: UserId,
    pub log: Event,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    AddOwner {
        user_id: UserId,
    },
    RemoveOwner {
        user_id: UserId,
    },
    AddNode {
        node_id: NodeId,
        content: Content,
    },
    RemoveNode(NodeId),
    PatchContent {
        node_id: NodeId,
        patch: ContentPatch,
    },
    PatchChildren {
        node_id: NodeId,
        patch: ChildrenPatch,
    },
    PatchAttribute {
        node_id: NodeId,
        patch: AttributePatch,
    },
    AddSnapshot {
        index: usize,
        snapshot: Snapshot,
    },
    AddFile(FileId),
    DeleteFile(FileId),
    // PatchFile
    AddCard {
        card_id: CardId,
        node_id: NodeId,
    },
    RemoveCard(CardId),
}
