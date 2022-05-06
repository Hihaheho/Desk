use std::collections::HashMap;

use deskc_ids::{CardId, FileId};
use dkernel_card::node::{Node, NodeId};
use dkernel_file::File;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Snapshot {
    pub asts: HashMap<CardId, Node>,
    pub files: HashMap<FileId, File>,
    pub references: HashMap<NodeId, Vec<NodeId>>,
}
