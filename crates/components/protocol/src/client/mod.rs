use futures::prelude::*;

use crate::{Command, Event};

pub struct Client<
    Tx: Sink<Command> + Send + Sync + 'static,
    Rx: Stream<Item = Event> + Send + Sync + 'static,
> {
    pub command_sender: Tx,
    pub event_receiver: Rx,
}

impl<Tx, Rx> From<(Tx, Rx)> for Client<Tx, Rx>
where
    Tx: Sink<Command> + Send + Sync + 'static,
    Rx: Stream<Item = Event> + Send + Sync + 'static,
{
    fn from((command_sender, event_receiver): (Tx, Rx)) -> Self {
        Self {
            command_sender,
            event_receiver,
        }
    }
}
