use crate::id::*;

#[derive(Clone, Copy, Debug, Hash)]
pub struct CardId(Id);

impl CardId {
    pub fn new() -> Self {
        Self(create_new_id())
    }
}
