use dkernel_ast::patch::{AttributePatch, ChildrenPatch, ContentPatch};
use dkernel_file::UserId;

pub trait LogRepository {
    fn poll(&mut self) -> Vec<LogEntry>;
    fn commit(&mut self, log: Vec<Log>);
}

pub struct LogEntry {
    pub user_id: UserId,
    pub log: Log,
}

pub enum Log {
    ContentPatch(ContentPatch),
    ChildrenPatch(ChildrenPatch),
    AttributePatch(AttributePatch),
}
