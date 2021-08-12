use bevy::prelude::*;
use client_websocket::connect;
use futures::future::ready;
use futures::prelude::*;
use lazy_static::lazy_static;
use protocol::{BoxClient, ClientName};
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Mutex;
use tracing::error;
use wasm_bindgen_futures::spawn_local;

const DEFAULT_CLIENT: ClientName =
    ClientName(Cow::Borrowed("desk-plugin-protocol: default client"));

lazy_static! {
    static ref CLIENTS: Mutex<HashMap<ClientName, BoxClient>> = Mutex::new(HashMap::new());
}

pub fn connect_websocket() {
    spawn_local(
        connect(DEFAULT_CLIENT, "ws://127.0.0.1:5000/ws".into()).then(|client| {
            ready(
                client
                    .map(|client| -> BoxClient { Box::new(client) })
                    .map(|client| {
                        CLIENTS.lock().unwrap().insert(DEFAULT_CLIENT, client);
                    })
                    .unwrap_or_else(|err| error!("{}", err)),
            )
        }),
    );
}

pub fn set_client(mut client_res: ResMut<Option<BoxClient>>) {
    let mut map = CLIENTS.lock().unwrap();
    if let Some(client) = map.remove(&DEFAULT_CLIENT) {
        *client_res = Some(client);
    }
}

pub fn block_on<T>(future: impl Future<Output = T> + 'static) {
    wasm_bindgen_futures::spawn_local(async { future.map(|_| ()).await });
}
