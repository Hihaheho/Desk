use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::primitives::*;
use crate::UserId;

#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AuthenticationErrorCode {
    UserNotFound,
    TokenExpired,
    TokenRevoked,
    InsufficientPermission,
    InvalidToken,
    InternalError,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AuthenticationError {
    pub error_code: AuthenticationErrorCode,
    pub message: String,
}

#[async_trait]
pub trait UserAuthenticationHandler {
    async fn authenticate(&self, token: &Token) -> Result<UserId, AuthenticationError>;
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use mock_it::Mock;

    #[derive(Clone)]
    pub struct MockUserAuthenticationHandler {
        pub authenticate: Mock<Token, Result<UserId, AuthenticationError>>,
    }

    impl Default for MockUserAuthenticationHandler {
        fn default() -> Self {
            Self {
                authenticate: Mock::new(Err(AuthenticationError {
                    error_code: AuthenticationErrorCode::InternalError,
                    message: "UserAuthenticationHandler.authenticate mock not found".into(),
                })),
            }
        }
    }

    #[async_trait]
    impl UserAuthenticationHandler for MockUserAuthenticationHandler {
        async fn authenticate(&self, token: &Token) -> Result<UserId, AuthenticationError> {
            self.authenticate.called(token.clone())
        }
    }
}
