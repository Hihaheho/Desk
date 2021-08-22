use std::env::var;
use std::time::Duration;
use tracing::*;

use eyre::Result;
use firestore_grpc::tonic::metadata::MetadataValue;
use firestore_grpc::tonic::transport::Channel;
use firestore_grpc::tonic::Request;
use firestore_grpc::v1::firestore_client::FirestoreClient;
use firestore_grpc::v1::{ListenRequest, ListenResponse};
use futures::channel::mpsc::channel;
use futures::prelude::*;
use lazy_static::lazy_static;
use parking_lot::RwLock;
use serde::Deserialize;
use tokio::time::sleep;

lazy_static! {
    static ref URL: String =
        var("URL").unwrap_or_else(|_| "https://firestore.googleapis.com".into());
    pub static ref PROJECT_ID: String = var("PROJECT_ID").unwrap();
    static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::builder()
        .pool_max_idle_per_host(1)
        .build()
        .unwrap();
    static ref TOKEN: RwLock<Option<String>> = RwLock::new(std::env::var("TOKEN").ok());
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u32,
    #[allow(dead_code)]
    token_type: String,
}

async fn update_token() -> Result<()> {
    loop {
        let token_response = HTTP_CLIENT
            .post("http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token?scopes=https://www.googleapis.com/auth/datastore")
            .header("Metadata-Flavor", "Google").send().await?;
        let token_res = token_response.json::<TokenResponse>().await?;
        *TOKEN.write() = Some(token_res.access_token);
        // divided by 2 for safety
        info!("access_token is updated");
        sleep(Duration::from_secs(token_res.expires_in as u64 / 2)).await;
    }
}
async fn get_client() -> Result<FirestoreClient<Channel>> {
    let endpoint = Channel::from_static(&URL);
    let channel = endpoint.connect().await?;

    let service = FirestoreClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert(
            "authorization",
            MetadataValue::from_str(&format!("Bearer {}", TOKEN.read().as_ref().unwrap())).unwrap(),
        );
        Ok(req)
    });
    Ok(service)
}

#[tokio::main]
pub async fn create_client() -> Result<(
    FirestoreClient<Channel>,
    impl Sink<ListenRequest> + Unpin + Send + Sync + 'static,
    impl Stream<Item = ListenResponse> + Unpin + Send + Sync + 'static,
)> {
    if TOKEN.read().is_none() {
        tokio::spawn(update_token());
    }
    let mut client = get_client().await?;

    let (listen_request_sender, rx) = channel(32);
    let listen_response_receiver = client.listen(rx).await?;
    Ok((
        client,
        listen_request_sender,
        listen_response_receiver
            .into_inner()
            .map(|response| response.unwrap()),
    ))
}
