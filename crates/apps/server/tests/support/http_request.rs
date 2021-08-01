use std::convert::TryInto;

use httpmock::{Method, MockRef, MockServer};
use hyper::{client::HttpConnector, Body, Client, Response, StatusCode, Uri};

use super::{builder, http_client, start_mock_server, Context};

impl Context {
    pub async fn execute_get<T: Into<String>>(&mut self, path: T) -> &mut Self {
        let base = self.get::<Uri>().unwrap().clone();
        let client: &mut Client<HttpConnector> = self.get_mut().unwrap();
        let response = client
            .get(
                Uri::builder()
                    .scheme(base.scheme().unwrap().clone())
                    .authority(base.authority().unwrap().clone())
                    .path_and_query(path.into())
                    .build()
                    .unwrap(),
            )
            .await
            .unwrap();
        self.insert(response);
        self
    }
    pub fn status_should_be<T: TryInto<StatusCode>>(&mut self, status: T) -> &mut Self {
        let response: &Response<Body> = self.get().unwrap();
        let status = status.try_into().map_err(|_| ()).unwrap();
        assert_eq!(response.status().as_u16(), status.as_u16());

        self
    }
}

#[tokio::test]
#[should_panic]
async fn status_should_be_fails() {
    builder()
        .before(start_mock_server)
        .before(http_client)
        .build()
        .run_async(|ctx| async move {
            let _mock = http_server_returns_200(ctx);
            ctx.execute_get("/ok").await
        })
        .await
        .status_should_be(500);
}

#[tokio::test]
async fn status_should_be() {
    builder()
        .before(start_mock_server)
        .before(http_client)
        .build()
        .run_async(|ctx| async move {
            let _mock = http_server_returns_200(ctx);
            ctx.execute_get("/ok").await
        })
        .await
        .status_should_be(200);
}

fn http_server_returns_200(ctx: &mut Context) -> MockRef {
    let server: &mut MockServer = ctx.get_mut().unwrap();
    server.mock(|when, then| {
        when.method(Method::GET).path("/ok");
        then.status(200);
    })
}
