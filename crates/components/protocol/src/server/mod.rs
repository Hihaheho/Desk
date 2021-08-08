mod before_login;
mod normal;

use crate::{primitives::*, Command, UserAuthenticationHandler};
use crate::{ErrorCode, Event};
use before_login::*;
use futures::{Sink, SinkExt};
use normal::*;

use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use tracing::error;

pub struct ServerContext<EventSender> {
    user_authentication_handler: Box<dyn UserAuthenticationHandler + Send + Sync>,
    event_sender: EventSender,
}

impl<T: Sink<Event> + Unpin + Send + Sync> ServerContext<T> {
    pub async fn send(&mut self, event: Event) {
        self.event_sender
            .send(event)
            .await
            .unwrap_or_else(|_err| error!("error to send an event"));
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ServerInput {
    Command {
        command: Command,
    },
    InboundCommand {
        local_user_id: RoomLocalUserId,
        command: Command,
    },
}

#[enum_dispatch(ServerState)]
#[derive(Debug, PartialEq, Clone)]
pub enum ServerStateSet {
    Normal,
    BeforeLogin,
}

#[async_trait]
#[enum_dispatch]
trait ServerState {
    async fn handle<T: Sink<Event> + Unpin + Send + Sync>(
        &self,
        context: &mut ServerContext<T>,
        input: &ServerInput,
    ) -> ServerStateSet;
}

pub async fn handle_unexpected_input<T: Sink<Event> + Unpin + Send + Sync>(
    context: &mut ServerContext<T>,
    input: &ServerInput,
) {
    if let ServerInput::Command { command: _ } = input {
        let event = Event::Error {
            code: ErrorCode::UnexpectedOperation,
            message: "unexpected".into(),
        };
        context.send(event).await;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::mock::MockUserAuthenticationHandler;
    use futures::channel::mpsc::channel;
    use futures::StreamExt;
    use uuid::Uuid;

    #[tokio::test]
    async fn unexpected_operation() {
        let auth = MockUserAuthenticationHandler::default();
        let (tx, mut rx) = channel::<Event>(1);

        let mut context = ServerContext {
            user_authentication_handler: Box::new(auth.clone()),
            event_sender: Box::new(tx),
        };
        let input = ServerInput::Command {
            command: Command::CreateRoom {
                room_name: "my room".into(),
            },
        };

        handle_unexpected_input(&mut context, &input).await;
        assert!(matches!(
            rx.next().await,
            Some(Event::Error {
                code: ErrorCode::UnexpectedOperation,
                ..
            })
        ));
    }

    #[tokio::test]
    async fn unexpected_inbound_operation() {
        let auth = MockUserAuthenticationHandler::default();
        let (mut tx, mut rx) = channel::<Event>(1);

        let mut context = ServerContext {
            user_authentication_handler: Box::new(auth.clone()),
            event_sender: Box::new(tx.clone()),
        };
        let input = ServerInput::InboundCommand {
            local_user_id: Uuid::new_v4().into(),
            command: Command::CreateRoom {
                room_name: "my room".into(),
            },
        };

        let irrelevent_event = Event::Error {
            code: ErrorCode::InternalError,
            message: "this is irrelevent".into(),
        };

        handle_unexpected_input(&mut context, &input).await;

        tx.send(irrelevent_event.clone()).await.unwrap();

        assert_eq!(rx.next().await, Some(irrelevent_event));
    }
}
