use axum::prelude::*;
use axum::ws::{ws, Message, WebSocket};
use eyre::Result;
use futures::channel::mpsc::channel;
use futures::future::{join_all, select_all};
use futures::stream::StreamExt;
use futures::{Sink, Stream};
use opentelemetry::sdk::export::trace::stdout;
use protocol::{unwrap_and_log, Channel, Command, Event};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::task::{JoinError, JoinHandle};

use tracing::error;
use user_authentication_firebase::FirebaseAuthentication;

use tracing_subscriber::layer::SubscriberExt;

use tracing_subscriber::Registry;

fn default_port() -> u16 {
    4000
}

#[derive(Deserialize, Debug)]
struct Config {
    #[serde(default = "default_port")]
    port: u16,
}

#[tokio::main]
pub async fn main() {
    let tracer = stdout::new_pipeline().install_simple();
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = Registry::default().with(telemetry);
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

    let (operation_sender, operation_receiver) = channel::<Command>(32);
    let (event_sender, event_receiver) = channel::<Event>(32);

    let receiver = receiver.filter_map(unwrap_and_log);

    let channel = Channel {};

    let auth = Box::new(FirebaseAuthentication {});

    let tasks = vec![
        tokio::spawn(recv_task(receiver, operation_sender)),
        tokio::spawn(channel.connect(auth, operation_receiver, event_sender)),
        tokio::spawn(send_task(event_receiver, sender)),
    ];

    match abort_all_for_one(tasks).await {
        Ok(_) => (),
        Err(err) => {
            error!("{}", err);
        }
    };
}

async fn recv_task(stream: impl Stream<Item = Message>, operations: impl Sink<Command>) {
    let _ = stream
        .map(|message| -> Result<Command> { Ok(serde_cbor::from_slice(message.as_bytes())?) })
        .filter_map(unwrap_and_log)
        .map(Ok)
        .forward(operations)
        .await;
}

async fn send_task(events: impl Stream<Item = Event> + Unpin, sink: impl Sink<Message>) {
    let _ = events
        .map(|operation| -> Result<Message> {
            Ok(Message::binary(serde_cbor::to_vec(&operation)?))
        })
        .filter_map(unwrap_and_log)
        .map(Ok)
        .forward(sink)
        .await;
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
