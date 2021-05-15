use crate::id::*;

pub struct BoxId(Id);

impl BoxId {
    pub fn new() -> Self {
        Self(create_new_id())
    }
}
