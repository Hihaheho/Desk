use crate::id::*;

pub struct TermId(Id);

impl TermId {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(create_new_id())
    }
}
