use axum::prelude::*;
use axum::ws::{ws, Message, WebSocket};
use eyre::Result;
use futures::future::{join_all, ready, select_all};
use futures::prelude::*;

use protocol::{Channel, Command};

use serde::Deserialize;
use std::net::SocketAddr;
use tokio::task::{JoinError, JoinHandle};

use tracing::{debug, error};
use user_authentication_firebase::FirebaseAuthentication;

use tracing_subscriber::layer::SubscriberExt;

use tracing_subscriber::{fmt, Registry};

fn default_port() -> u16 {
    5000
}

#[derive(Deserialize, Debug)]
struct Config {
    #[serde(default = "default_port")]
    port: u16,
}

#[tokio::main]
pub async fn main() {
    let subscriber = Registry::default().with(fmt::Layer::new().json());
    tracing::subscriber::set_global_default(subscriber).expect("set_global_default failed");

    let config = envy::from_env::<Config>().unwrap();
    let app = route("/ws", ws(handle_socket));

    hyper::Server::bind(&SocketAddr::from(([0, 0, 0, 0], config.port)))
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_socket(socket: WebSocket) {
    let (sender, receiver) = socket.split();
    let channel = Channel {};
    let auth = Box::new(FirebaseAuthentication {});

    debug!("start ws connection");
    channel
        .connect(
            auth,
            receiver
                .map(|message| message.unwrap())
                .map(|message| -> Command { serde_cbor::from_slice(message.as_bytes()).unwrap() }),
            sender
                .sink_map_err(|err| err.to_string())
                .with(|operation| {
                    ready(
                        serde_cbor::to_vec(&operation)
                            .map(Message::binary)
                            .map_err(|err| {
                                let err = err.to_string();
                                error!("{}", err);
                                err
                            }),
                    )
                }),
        )
        .await;
    debug!("finish ws connection");
}

pub async fn abort_all_for_one<T, I>(tasks: I) -> Result<T, JoinError>
where
    I: IntoIterator<Item = JoinHandle<T>>,
{
    let (result, _, tasks) = select_all(tasks.into_iter()).await;
    tasks.iter().for_each(|task| task.abort());
    let _ = join_all(tasks).await;
    result
}
