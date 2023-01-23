use components::{event::Event, projection::Projection};
use downcast_rs::{impl_downcast, Downcast};

pub trait State: Downcast {
    fn handle_event(&mut self, projection: &Projection, event: &Event);
}

impl_downcast!(State);
