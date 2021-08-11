#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
pub use wasm::*;

use thiserror::Error;

pub struct WebSocketClient<Tx, Rx> {
    tx: Tx,
    rx: Rx,
}

impl<Tx, Rx> WebSocketClient<Tx, Rx> {
    pub fn split(self) -> (Tx, Rx) {
        (self.tx, self.rx)
    }
}

#[non_exhaustive]
#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum SinkError {
    // TODO: Refine
    #[error("send failed {0}")]
    Send(String),
}
