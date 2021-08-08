use std::borrow::Cow;

use eyre::Result;
use protocol::futures::future::{ready, Ready};
use protocol::futures::{prelude::*, Sink, Stream};
use protocol::{unwrap_and_log, Client, Command, Event};
use tracing::error;

#[cfg(not(target_arch = "wasm32"))]
pub async fn connect<'a, T: Into<Cow<'a, str>>>(
    addr: T,
) -> Result<Client<impl Sink<Command>, impl Stream<Item = Event>>> {
    use tokio_tungstenite::connect_async;
    use tokio_tungstenite::tungstenite::Message;

    let str: Cow<'a, str> = addr.into();
    let stream = connect_async(str.as_ref()).await?;
    let (tx, rx) = stream.0.split();
    let rx = rx
        .map(|message| -> Result<Event> { Ok(serde_cbor::from_slice(&message?.into_data())?) })
        .filter_map(unwrap_and_log);
    let tx = tx.with(|command: Command| -> Ready<Result<Message>> {
        match serde_cbor::to_vec(&command) {
            Ok(vec) => ready(Ok(Message::binary(vec))),
            Err(err) => {
                error!("{}", err);
                ready(Err(err.into()))
            }
        }
    });

    Ok((tx, rx).into())
}

#[cfg(target_arch = "wasm32")]
pub async fn connect<'a, T: Into<Cow<'a, str>>>(
    addr: T,
) -> Result<Client<impl Sink<Command>, impl Stream<Item = Event>>> {
    use ws_stream_wasm::WsMessage;
    use ws_stream_wasm::WsMeta;

    let str: Cow<'a, str> = addr.into();
    let (_, wsio) = WsMeta::connect(str.as_ref(), None).await?;
    let (tx, rx) = wsio.split();
    let rx = rx
        .map(|message| -> Result<Event> { Ok(serde_cbor::from_slice(message.as_ref())?) })
        .filter_map(unwrap_and_log);
    let tx = tx.with(|command: Command| -> Ready<Result<WsMessage>> {
        match serde_cbor::to_vec(&command) {
            Ok(vec) => ready(Ok(WsMessage::Binary(vec))),
            Err(err) => {
                error!("{}", err);
                ready(Err(err.into()))
            }
        }
    });

    Ok((tx, rx).into())
}
