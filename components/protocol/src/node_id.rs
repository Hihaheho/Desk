use crate::id::*;

pub struct NodeId(Id);

impl NodeId {
    pub fn new() -> Self {
        Self(create_new_id())
    }
}
