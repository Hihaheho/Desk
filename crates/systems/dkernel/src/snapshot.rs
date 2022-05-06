use std::collections::{HashMap, HashSet};

use deskc_ids::{CardId, FileId};
use dkernel_card::{
    file::File,
    flat_node::FlatNode,
    node::{Node, NodeId},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Snapshot {
    pub asts: HashMap<NodeId, FlatNode>,
    pub files: HashMap<FileId, File>,
    pub references: HashMap<NodeId, HashSet<NodeId>>,
}
