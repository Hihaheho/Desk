use crate::id::*;

pub struct NodeId(Id);

impl NodeId {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(create_new_id())
    }
}
