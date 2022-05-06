use std::collections::{HashMap, HashSet};

use deskc_ids::{CardId, FileId};
use dkernel_card::{
    file::File,
    flat_node::FlatNode,
    node::{Node, NodeId},
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Snapshot {
    pub nodes: HashMap<NodeId, FlatNode>,
    pub files: HashMap<FileId, File>,
    references: HashMap<NodeId, HashSet<NodeId>>,
    pub cards: HashSet<CardId>,
}

impl Snapshot {}
