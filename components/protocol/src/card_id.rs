use crate::id::*;

#[derive(Clone, Copy, Debug, Hash)]
pub struct CardId(Id);

impl ToString for CardId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl CardId {
    pub fn new() -> Self {
        Self(create_new_id())
    }
}
