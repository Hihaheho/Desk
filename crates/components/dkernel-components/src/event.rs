use crate::{
    content::Content,
    patch::{AttributePatch, ChildrenPatch, ContentPatch, FilePatch},
    rules::{Rules, SpaceOperation},
    user::UserId,
};
use deskc_ids::{FileId, NodeId};

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
    UpdateRule {
        rules: Rules<SpaceOperation>,
    },
    AddNode {
        node_id: NodeId,
        file_id: FileId,
        content: Content,
    },
    RemoveNode {
        node_id: NodeId,
    },
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
        snapshot: Box<Snapshot>,
    },
    AddFile(FileId),
    DeleteFile(FileId),
    PatchFile {
        file_id: FileId,
        patch: FilePatch,
    },
}
