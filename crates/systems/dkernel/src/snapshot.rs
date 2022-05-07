use std::collections::{HashMap, HashSet};

use deskc_ids::{CardId, FileId, UserId};
use dkernel_card::{file::File, flat_node::FlatNode, node::NodeId};

use crate::repository::LogEntry;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Snapshot {
    pub owners: HashSet<UserId>,
    pub nodes: HashMap<NodeId, FlatNode>,
    pub files: HashMap<FileId, File>,
    pub card_files: HashMap<CardId, FileId>,
}

impl Snapshot {
    pub fn handle_log_entry(&mut self, snapshot: &Snapshot, log: &LogEntry) {}
    pub fn allowed_log_entry(&self, log: &LogEntry) -> bool {
        true
    }
}
