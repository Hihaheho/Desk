use futures::{Sink, Stream};

use crate::{Event, Operation};

pub struct Channel {}

impl Channel {
    pub async fn connect(
        self,
        _operation_stream: impl Stream<Item = Operation>,
        _event_emitter: impl Sink<Event>,
    ) {
    }
}

#[cfg(test)]
mod test {}
