use crate::ErrorCode;
use crate::{handle_unexpected_input, Login};
use crate::{Command, Event};
use crate::{ServerContext, ServerInput, ServerStateDispatcher};

use super::normal::Normal;
use super::ServerState;
use async_trait::async_trait;

#[derive(Debug, PartialEq, Clone)]
pub struct BeforeLogin {}

#[async_trait]
impl ServerState for BeforeLogin {
    async fn handle(
        self,
        context: &mut (impl ServerContext + Send + Sync),
        input: &ServerInput,
    ) -> ServerStateDispatcher {
        match &input {
            ServerInput::Command(Command::Login(Login { token })) => {
                let auth = context.authenticate(token).await;
                match auth {
                    Ok(user_id) => {
                        context
                            .send_event(Event::LoggedIn {
                                user_id: user_id.clone(),
                            })
                            .await;
                        return Normal::new(user_id).into();
                    }
                    Err(error) => {
                        let event = Event::Error {
                            code: ErrorCode::Authentication(error.error_code),
                            message: error.message,
                        };
                        context.send_event(event).await;
                    }
                }
            }
            unexpected => handle_unexpected_input(context, unexpected).await,
        }
        self.into()
    }
}

#[cfg(test)]
mod test {
    use futures::channel::mpsc::channel;

    use futures::StreamExt;
    use uuid::Uuid;

    use super::*;
    use crate::mock::MockUserAuthenticationHandler;
    use crate::AuthenticationErrorCode;
    use crate::{primitives::*, SinkAndStreamServerContext};
    use crate::{AuthenticationError, ErrorCode};

    #[tokio::test]
    async fn handle_login_operation() {
        let state: ServerStateDispatcher = BeforeLogin {}.into();
        let auth = MockUserAuthenticationHandler::default();

        let token: Token = vec![1, 2, 3].into();
        let user_id = UserId(Uuid::new_v4());

        auth.authenticate
            .given(token.clone())
            .will_return(Ok(user_id.clone()));

        let (event_sender, mut event_rx) = channel(1);
        let (entrance_command_sender, mut _entrance_rx) = channel(1);

        let mut context = SinkAndStreamServerContext {
            user_authentication_handler: Box::new(auth.clone()),
            event_sender,
            entrance_command_sender,
        };
        let input = ServerInput::Command(Command::Login(Login { token }));
        assert_eq!(
            state.handle(&mut context, &input).await,
            Normal::new(user_id.clone()).into()
        );
        assert_eq!(event_rx.next().await, Some(Event::LoggedIn { user_id }));
    }

    #[tokio::test]
    async fn failed_to_login() {
        let state: ServerStateDispatcher = BeforeLogin {}.into();
        let auth = MockUserAuthenticationHandler::default();

        let token: Token = vec![1, 2, 3].into();

        auth.authenticate
            .given(token.clone())
            .will_return(Err(AuthenticationError {
                error_code: AuthenticationErrorCode::InsufficientPermission,
                message: "error".into(),
            }));

        let (event_sender, mut event_rx) = channel(1);
        let (entrance_command_sender, mut _entrance_rx) = channel(1);

        let mut context = SinkAndStreamServerContext {
            user_authentication_handler: Box::new(auth.clone()),
            event_sender,
            entrance_command_sender,
        };
        let input = ServerInput::Command(Command::Login(Login { token }));

        assert_eq!(
            state.handle(&mut context, &input).await,
            BeforeLogin {}.into()
        );
        assert_eq!(
            event_rx.next().await,
            Some(Event::Error {
                code: ErrorCode::Authentication(AuthenticationErrorCode::InsufficientPermission),
                message: "error".into()
            })
        );
    }
}
