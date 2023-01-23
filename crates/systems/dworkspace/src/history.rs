use components::{event::Event, projection::Projection};

#[derive(Default)]
pub struct History {}

impl History {
    pub fn handle_event(&mut self, _snapshot: &Projection, _log: &Event) {}
}
