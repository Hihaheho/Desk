use std::borrow::Cow;

use futures::sink::SinkMapErr;
use futures::stream::{Map, SplitSink, SplitStream};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::{Error, Message};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

use crate::{SinkError, WebSocketClient};

use eyre::Result;
use futures::future::{ready, Ready};
use futures::prelude::*;

type Bytes = Vec<u8>;

pub type Rx = Map<
    SplitStream<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>>,
    fn(Result<Message, Error>) -> Result<Bytes>,
>;

pub type Tx = sink::With<
    SinkMapErr<
        SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
        fn(Error) -> SinkError,
    >,
    Message,
    Bytes,
    Ready<Result<Message, SinkError>>,
    fn(Bytes) -> Ready<Result<Message, SinkError>>,
>;

pub async fn connect<'a, T: Into<Cow<'a, str>>>(addr: T) -> Result<WebSocketClient<Tx, Rx>> {
    let str: Cow<'a, str> = addr.into();
    let (stream, _response) = connect_async(str.as_ref()).await?;
    let (tx, rx) = stream.split();
    let rx = rx.map(bytes as fn(_) -> _);
    let tx = tx
        .sink_map_err(err as fn(_) -> _)
        .with(message as fn(_) -> _);

    Ok(WebSocketClient { tx, rx })
}

fn bytes(message: Result<Message, Error>) -> Result<Bytes> {
    Ok(message?.into_data())
}

fn err(err: Error) -> SinkError {
    SinkError::Send(err.to_string())
}

fn message(bytes: Bytes) -> Ready<Result<Message, SinkError>> {
    ready(Ok(Message::binary(bytes)))
}
