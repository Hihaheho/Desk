use crate::server::normal::Normal;
use crate::ErrorCode;
use crate::{handle_unexpected_input, Login};
use crate::{Command, Event};
use crate::{ServerContext, ServerInput, ServerStateSet};

use super::ServerState;
use async_trait::async_trait;
use futures::Sink;

#[derive(Debug, PartialEq, Clone)]
pub struct BeforeLogin {}

#[async_trait]
impl ServerState for BeforeLogin {
    async fn handle<T: Sink<Event> + Unpin + Send + Sync>(
        &self,
        context: &mut ServerContext<T>,
        input: &ServerInput,
    ) -> ServerStateSet {
        match &input {
            ServerInput::Command {
                command: Command::Login(Login { token }),
            } => {
                let auth = context
                    .user_authentication_handler
                    .authenticate(token)
                    .await;
                match auth {
                    Ok(user_id) => return Normal { user_id }.into(),
                    Err(error) => {
                        let event = Event::Error {
                            code: ErrorCode::Authentication(error.error_code),
                            message: error.message,
                        };
                        context.send(event).await;
                    }
                }
            }
            unexpected => handle_unexpected_input(context, unexpected).await,
        }
        self.clone().into()
    }
}

#[cfg(test)]
mod test {
    use futures::channel::mpsc::channel;
    use futures::sink::drain;
    use futures::StreamExt;
    use uuid::Uuid;

    use super::*;
    use crate::mock::MockUserAuthenticationHandler;
    use crate::primitives::*;
    use crate::AuthenticationErrorCode;
    use crate::{AuthenticationError, ErrorCode};

    #[tokio::test]
    async fn handle_login_operation() {
        let state: ServerStateSet = BeforeLogin {}.into();
        let auth = MockUserAuthenticationHandler::default();

        let token: Token = vec![1, 2, 3].into();
        let user_id = UserId(Uuid::new_v4());

        auth.authenticate
            .given(token.clone())
            .will_return(Ok(user_id.clone()));

        let mut context = ServerContext {
            user_authentication_handler: Box::new(auth.clone()),
            event_sender: Box::new(drain()),
        };
        let input = ServerInput::Command {
            command: Command::Login(Login { token }),
        };
        assert_eq!(
            state.handle(&mut context, &input).await,
            Normal { user_id }.into()
        );
    }

    #[tokio::test]
    async fn failed_to_login() {
        let state: ServerStateSet = BeforeLogin {}.into();
        let auth = MockUserAuthenticationHandler::default();

        let token: Token = vec![1, 2, 3].into();

        auth.authenticate
            .given(token.clone())
            .will_return(Err(AuthenticationError {
                error_code: AuthenticationErrorCode::InsufficientPermission,
                message: "error".into(),
            }));

        let (tx, mut rx) = channel::<Event>(1);

        let mut context = ServerContext {
            user_authentication_handler: Box::new(auth.clone()),
            event_sender: Box::new(tx),
        };
        let input = ServerInput::Command {
            command: Command::Login(Login { token }),
        };

        assert_eq!(
            state.handle(&mut context, &input).await,
            BeforeLogin {}.into()
        );
        assert_eq!(
            rx.next().await,
            Some(Event::Error {
                code: ErrorCode::Authentication(AuthenticationErrorCode::InsufficientPermission),
                message: "error".into()
            })
        );
    }
}
