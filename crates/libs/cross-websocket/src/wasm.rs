use futures::sink::{SinkMapErr, With};
use futures::stream::{Map, SplitSink, SplitStream};
use std::borrow::Cow;
use ws_stream_wasm::{WsErr, WsMessage, WsMeta, WsStream};

use crate::{SinkError, WebSocketClient};
use eyre::Result;
use futures::future::{ready, Ready};
use futures::prelude::*;

pub type Bytes = Vec<u8>;

pub type Rx = Map<SplitStream<WsStream>, fn(WsMessage) -> Result<Bytes>>;
pub type Tx = With<
    SinkMapErr<SplitSink<WsStream, WsMessage>, fn(WsErr) -> SinkError>,
    WsMessage,
    Bytes,
    Ready<Result<WsMessage, SinkError>>,
    fn(Bytes) -> Ready<Result<WsMessage, SinkError>>,
>;

pub async fn connect<'a, T: Into<Cow<'a, str>>>(addr: T) -> Result<WebSocketClient<Tx, Rx>> {
    let str: Cow<'a, str> = addr.into();
    let (_, wsio) = WsMeta::connect(str.as_ref(), None).await?;
    let (tx, rx) = wsio.split();
    let rx = rx.map(bytes as fn(_) -> _);
    let tx = tx
        .sink_map_err(err as fn(_) -> _)
        .with(message as fn(_) -> _);

    Ok(WebSocketClient { tx, rx })
}

fn bytes(message: WsMessage) -> Result<Bytes> {
    Ok(message.into())
}

fn err(err: WsErr) -> SinkError {
    SinkError::Send(err.to_string())
}

fn message(bytes: Bytes) -> Ready<Result<WsMessage, SinkError>> {
    ready(Ok(WsMessage::Binary(bytes)))
}
