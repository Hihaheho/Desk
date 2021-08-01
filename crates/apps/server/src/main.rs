use axum::prelude::*;
use serde::Deserialize;
use std::net::SocketAddr;

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
    let config = envy::from_env::<Config>().unwrap();
    let app = route("/", get(|| async { "Hello, World!" }));

    hyper::Server::bind(&SocketAddr::from(([0, 0, 0, 0], config.port)))
        .serve(app.into_make_service())
        .await
        .unwrap();
}
