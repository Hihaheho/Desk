use std::borrow::Cow;
use ws_stream_wasm::{WsMessage, WsMeta};

use crate::{SinkError, WebSocketClient};
use eyre::Result;
use futures::future::{ready, Ready};
use futures::{prelude::*, Sink, Stream};

type Bytes = Vec<u8>;

pub async fn connect<'a, T: Into<Cow<'a, str>>>(
    addr: T,
) -> Result<
    WebSocketClient<
        impl Sink<Bytes, Error = SinkError> + Send + Sync + Unpin + 'static,
        impl Stream<Item = Result<Bytes>> + Send + Sync + Unpin + 'static,
    >,
> {
    let str: Cow<'a, str> = addr.into();
    let (_, wsio) = WsMeta::connect(str.as_ref(), None).await?;
    let (tx, rx) = wsio.split();
    let rx = rx.map(|message| -> Result<Bytes> { Ok(message.into()) });
    let tx = tx
        .sink_map_err(|err| SinkError::Send(err.to_string()))
        .with(|bytes: Bytes| -> Ready<Result<WsMessage, SinkError>> {
            ready(Ok(WsMessage::Binary(bytes)))
        });

    Ok(WebSocketClient { tx, rx })
}
