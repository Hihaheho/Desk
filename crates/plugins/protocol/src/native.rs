use bevy::prelude::*;
use client_websocket::connect;
use futures::Future;
use futures_lite::future;
use protocol::Clients;

use crate::DEFAULT_CLIENT;

pub fn connect_websocket(mut clients: ResMut<Clients>) {
    use std::mem::forget;
    use tokio::runtime::Runtime;

    let rt = Runtime::new().expect("failed to start a runtime");

    // Spawn the root task
    rt.block_on(connect("ws://127.0.0.1:5000/ws".into()))
        .map(|client| {
            clients.insert(DEFAULT_CLIENT, Box::new(client));
        })
        .unwrap_or_else(|err| error!("{}", err));

    forget(rt);
}

pub fn block_on<T>(future: impl Future<Output = T>) {
    future::block_on(future);
}
