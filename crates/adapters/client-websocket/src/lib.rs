#[cfg(not(target_arch = "wasm32"))]
mod native;

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
pub use wasm::*;

mod next_vec;
pub use next_vec::*;

use eyre::Result;
use protocol::futures::future::{ready, Ready};
use protocol::futures::{prelude::*, Stream};
use protocol::{unwrap_and_log, Command, Event};
use tracing::error;

fn event_receiver(
    rx: impl Stream<Item = Result<Vec<u8>>> + Send + Sync + 'static + Unpin,
) -> impl Stream<Item = Event> + Send + Sync + 'static + Unpin {
    rx.map(|bytes| -> Result<Event> { Ok(serde_cbor::from_slice(&bytes?)?) })
        .filter_map(unwrap_and_log!())
}

fn command_sender(
    tx: impl Sink<Vec<u8>, Error = String> + Clone + Send + Sync + 'static + Unpin,
) -> impl Sink<Command, Error = String> + Clone + Send + Sync + 'static + Unpin {
    tx.with(|command: Command| -> Ready<Result<Vec<u8>, String>> {
        match serde_cbor::to_vec(&command) {
            Ok(vec) => ready(Ok(vec)),
            Err(err) => {
                error!("{}", err);
                ready(Err(err.to_string()))
            }
        }
    })
}
