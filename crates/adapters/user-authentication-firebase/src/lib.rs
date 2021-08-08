use protocol::{UserAuthenticationHandler, UserId};

use async_trait::async_trait;
use uuid::Uuid;

pub struct FirebaseAuthentication {}

#[async_trait]
impl UserAuthenticationHandler for FirebaseAuthentication {
    async fn authenticate(
        &self,
        _token: &protocol::Token,
    ) -> Result<protocol::UserId, protocol::AuthenticationError> {
        // TODO
        Ok(UserId(Uuid::new_v4()))
    }
}
