use deskc_ids::FileId;

use crate::rules::{NodeOperation, Rules};

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct File {
    pub name: Option<String>,
    pub children: Vec<FileId>,
    pub rules: Rules<NodeOperation>,
}
