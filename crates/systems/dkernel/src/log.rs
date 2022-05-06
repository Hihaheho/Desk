use dkernel_card::{
    node::NodeId,
    patch::{AttributePatch, ChildrenPatch, ContentPatch},
};

pub trait LogRepository {
    fn poll(&mut self) -> Vec<LogEntry>;
    fn commit(&mut self, log: Vec<Log>);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogEntry {
    pub user_id: UserId,
    pub log: Log,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Log {
    ContentPatch {
        node_id: NodeId,
        patch: ContentPatch,
    },
    ChildrenPatch {
        node_id: NodeId,
        patch: ChildrenPatch,
    },
    AttributePatch {
        node_id: NodeId,
        patch: AttributePatch,
    },
}
