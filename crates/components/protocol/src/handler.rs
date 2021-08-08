use futures::{Sink, Stream};

use crate::{Command, Event};

pub struct Channel {}

impl Channel {
    pub async fn connect(
        self,
        _operation_stream: impl Stream<Item = Command>,
        _event_emitter: impl Sink<Event>,
    ) {
    }
}

#[cfg(test)]
mod test {}
