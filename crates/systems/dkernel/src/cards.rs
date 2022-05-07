use std::collections::{HashMap, HashSet};

use deskc_ids::{CardId, FileId};
use dkernel_card::node::NodeId;

use crate::{repository::LogEntry, snapshot::Snapshot};

#[derive(Default)]
pub struct Cards {
    references: HashMap<NodeId, HashSet<NodeId>>,
    pub cards: HashMap<CardId, HashSet<NodeId>>,
}

impl Cards {
    pub fn handle_log_entry(&mut self, snapshot: &Snapshot, log: &LogEntry) {}
}
