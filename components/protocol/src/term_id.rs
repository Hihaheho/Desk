use crate::id::*;

pub struct TermId(Id);

impl TermId {
    pub fn new() -> Self {
        Self(create_new_id())
    }
}
