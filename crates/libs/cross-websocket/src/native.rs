use std::borrow::Cow;

use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

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
    let (stream, _response) = connect_async(str.as_ref()).await?;
    let (tx, rx) = stream.split();
    let rx = rx.map(|message| -> Result<Bytes> { Ok(message?.into_data()) });
    let tx = tx
        .sink_map_err(|err| SinkError::Send(err.to_string()))
        .with(|bytes: Bytes| -> Ready<Result<Message, SinkError>> {
            ready(Ok(Message::binary(bytes)))
        });

    Ok(WebSocketClient { tx, rx })
}
