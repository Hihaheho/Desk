use crate::id::*;

pub struct BoxId(Id);

impl BoxId {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(create_new_id())
    }
}
