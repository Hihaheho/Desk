use crate::support::{builder, http_client, start_desk_server};

#[tokio::test]
async fn returns_not_found() {
    builder()
        .before(start_desk_server)
        .before(http_client)
        .build()
        .execute_get("/unknown-path")
        .await
        .status_should_be(404);
}
