use std::any::Any;

use components::{event::Event, snapshot::Snapshot};
use downcast_rs::{impl_downcast, Downcast};

pub trait State: Downcast {
    fn handle_event(&mut self, snapshot: &Snapshot, log: &Event);
}

impl_downcast!(State);
