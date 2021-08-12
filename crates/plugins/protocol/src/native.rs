use bevy::prelude::*;
use client_websocket::connect;
use futures::Future;
use futures_lite::future;
use protocol::BoxClient;

pub fn connect_websocket(mut client_res: ResMut<Option<BoxClient>>) {
    use std::mem::forget;
    use tokio::runtime::Runtime;

    let rt = Runtime::new().expect("failed to start a runtime");

    // Spawn the root task
    rt.block_on(connect("ws://127.0.0.1:5000/ws".into()))
        .map(|client| {
            *client_res = Some(Box::new(client));
        })
        .unwrap_or_else(|err| error!("{}", err));

    forget(rt);
}

pub fn block_on<T>(future: impl Future<Output = T>) {
    future::block_on(future);
}
