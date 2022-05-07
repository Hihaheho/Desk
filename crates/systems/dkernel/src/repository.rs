use deskc_ids::{FileId, UserId};
use dkernel_card::{
    content::Content,
    file::File,
    node::NodeId,
    patch::{AttributePatch, ChildrenPatch, ContentPatch},
};

use crate::snapshot::Snapshot;

pub trait Repository {
    fn poll(&mut self) -> Vec<LogEntry>;
    fn commit(&mut self, log: Log);
    fn add_owner(&mut self, user_id: UserId);
    fn remove_owner(&mut self, user_id: UserId);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogEntry {
    pub index: usize,
    pub user_id: UserId,
    pub log: Log,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Log {
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
    AddFile {
        file_id: FileId,
        content: File,
    },
}
