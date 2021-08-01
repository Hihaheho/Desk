use axum::prelude::*;
use clap::Clap;
use std::net::SocketAddr;

#[derive(Clap)]
struct Opts {
    #[clap(short, long, default_value = "4000")]
    port: u16,
}

#[tokio::main]
pub async fn main() {
    let opts = Opts::parse();
    let app = route("/", get(|| async { "Hello, World!" }));

    hyper::Server::bind(&SocketAddr::from(([0, 0, 0, 0], opts.port)))
        .serve(app.into_make_service())
        .await
        .unwrap();
}
