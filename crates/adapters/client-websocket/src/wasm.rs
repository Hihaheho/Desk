use crate::{command_sender, event_receiver};
use async_trait::async_trait;
use eyre::Result;
use lazy_static::lazy_static;
use protocol::futures::channel::mpsc::channel;
use protocol::futures::future::ready;
use protocol::futures::prelude::*;
use protocol::{Client, ClientName, Command, Event};
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref EVENTS: Mutex<HashMap<ClientName, Vec<Event>>> = Mutex::new(HashMap::default());
}

pub struct WebSocketClient<Tx> {
    client_name: ClientName,
    command_sender: Tx,
}

#[async_trait]
impl<Tx> Client for WebSocketClient<Tx>
where
    Tx: Sink<Command, Error = String> + Clone + Send + Sync + Unpin + 'static,
{
    fn sender(&self) -> Box<dyn Sink<Command, Error = String> + Send + Sync + Unpin + 'static> {
        Box::new(self.command_sender.clone())
    }

    fn poll_once(&mut self) -> Option<Vec<Event>> {
        let mut map = EVENTS.lock().unwrap();
        let events = map.get_mut(&self.client_name).unwrap();
        let result = events.clone();
        events.clear();
        events.truncate(10);
        return Some(result);
    }
}

pub async fn connect(
    client_name: ClientName,
    url: String,
) -> Result<
    WebSocketClient<impl Sink<Command, Error = String> + Clone + Send + Sync + Unpin + 'static>,
> {
    let (tx, rx) = cross_websocket::connect(url).await?.split();
    let (tx_clone, rx_clone) = channel::<Vec<u8>>(32);
    wasm_bindgen_futures::spawn_local(rx_clone.map(Ok).forward(tx).map(|_| ()));
    let event_receiver = event_receiver(rx);
    let result = Ok(WebSocketClient {
        client_name: client_name.clone(),
        command_sender: command_sender(tx_clone.sink_map_err(|err| err.to_string())),
    });
    EVENTS
        .lock()
        .unwrap()
        .insert(client_name.clone(), Vec::new());
    wasm_bindgen_futures::spawn_local(event_receiver.for_each(move |event| {
        ready(
            EVENTS
                .lock()
                .unwrap()
                .get_mut(&client_name)
                .unwrap()
                .push(event),
        )
    }));
    result
}
