mod before_login;
mod normal;
mod room;
mod user;

use crate::{ErrorCode, Event, ServerContext, ServerInput};
pub(in crate::server_state) use before_login::*;
pub(in crate::server_state) use normal::*;
pub(in crate::server_state) use room::*;

use async_trait::async_trait;
use enum_dispatch::enum_dispatch;

#[enum_dispatch(ServerState)]
#[derive(Debug, PartialEq, Clone)]
pub enum ServerStateDispatcher {
    Normal,
    BeforeLogin,
}

impl Default for ServerStateDispatcher {
    fn default() -> Self {
        Self::BeforeLogin(BeforeLogin {})
    }
}

#[async_trait]
#[enum_dispatch]
pub trait ServerState {
    async fn handle(
        self,
        context: &mut (impl ServerContext + Send + Sync),
        input: &ServerInput,
    ) -> ServerStateDispatcher;
}

pub async fn handle_unexpected_input(
    context: &mut (impl ServerContext + Send + Sync),
    input: &ServerInput,
) {
    if let ServerInput::Command(_) = input {
        let event = Event::Error {
            code: ErrorCode::UnexpectedOperation,
            message: "unexpected".into(),
        };
        context.send_event(event).await;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::mock::MockUserAuthenticationHandler;
    use crate::{Command, SinkAndStreamServerContext};
    use futures::channel::mpsc::channel;
    use futures::StreamExt;

    #[tokio::test]
    async fn unexpected_operation() {
        let auth = MockUserAuthenticationHandler::default();
        let (event_sender, mut event_rx) = channel(1);
        let (entrance_command_sender, _entrance_rx) = channel(1);

        let mut context = SinkAndStreamServerContext {
            user_authentication_handler: Box::new(auth.clone()),
            event_sender,
            entrance_command_sender,
        };
        let input = ServerInput::Command(Command::CreateRoom {
            room_name: "my room".into(),
        });

        handle_unexpected_input(&mut context, &input).await;
        assert!(matches!(
            event_rx.next().await,
            Some(Event::Error {
                code: ErrorCode::UnexpectedOperation,
                ..
            })
        ));
    }
}
