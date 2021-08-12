use async_trait::async_trait;

use eyre::Result;
use protocol::futures::channel::mpsc::channel;

use protocol::futures::{prelude::*, Sink, Stream};
use protocol::{Client, Command, Event};

use crate::{command_sender, event_receiver, NextVec};

pub struct WebSocketClient<Tx, Rx> {
    command_sender: Tx,
    event_receiver: Rx,
}

#[async_trait]
impl<Tx, Rx> Client for WebSocketClient<Tx, Rx>
where
    Tx: Sink<Command, Error = String> + Clone + Send + Sync + Unpin + 'static,
    Rx: Stream<Item = Event> + Send + Sync + Unpin + 'static,
{
    fn sender(&self) -> Box<dyn Sink<Command, Error = String> + Send + Sync + Unpin + 'static> {
        Box::new(self.command_sender.clone())
    }

    fn poll_once(&mut self) -> Option<Vec<Event>> {
        futures_lite::future::block_on(NextVec(&mut self.event_receiver))
    }
}

pub async fn connect(
    url: String,
) -> Result<
    WebSocketClient<
        impl Sink<Command, Error = String> + Clone + Send + Sync + Unpin + 'static,
        impl Stream<Item = Event> + Send + Sync + Unpin + 'static,
    >,
> {
    let (tx, rx) = cross_websocket::connect(url).await?.split();
    let (tx_clone, rx_clone) = channel::<Vec<u8>>(32);
    tokio::spawn(rx_clone.map(Ok).forward(tx));

    Ok(WebSocketClient {
        command_sender: command_sender(tx_clone.sink_map_err(|err| err.to_string())),
        event_receiver: event_receiver(rx),
    })
}
