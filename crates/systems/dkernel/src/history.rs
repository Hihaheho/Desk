use crate::{event::EventEntry, snapshot::Snapshot};

#[derive(Default)]
pub struct History {}

impl History {
    pub fn handle_log_entry(&mut self, snapshot: &Snapshot, log: &EventEntry) {}
}
