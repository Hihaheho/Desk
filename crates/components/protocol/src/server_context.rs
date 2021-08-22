use crate::{primitives::*, AuthenticationError, Command, Event, UserAuthenticationHandler};
use async_trait::async_trait;
use futures::{Sink, SinkExt};
use tracing::error;

pub struct SinkAndStreamServerContext<T, U> {
    pub user_authentication_handler: Box<dyn UserAuthenticationHandler + Send + Sync>,
    pub event_sender: T,
    pub entrance_command_sender: U,
}

#[async_trait]
pub trait ServerContext {
    async fn send_event(&mut self, event: Event);
    async fn authenticate(&mut self, token: &Token) -> Result<UserId, AuthenticationError>;
}

#[async_trait]
impl<T, U> ServerContext for SinkAndStreamServerContext<T, U>
where
    T: Sink<Event> + Unpin + Send + Sync,
    U: Sink<InboundEntranceCommand> + Unpin + Send + Sync,
{
    async fn send_event(&mut self, event: Event) {
        self.event_sender
            .send(event)
            .await
            .unwrap_or_else(|_err| error!("error to send an event"));
    }

    async fn authenticate(&mut self, token: &Token) -> Result<UserId, AuthenticationError> {
        self.user_authentication_handler.authenticate(token).await
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ServerInput {
    Command(Command),
    InboundEntranceCommand(InboundEntranceCommand),
}

#[derive(Debug, PartialEq, Clone)]
pub struct InboundEntranceCommand {
    pub local_user_id: RoomLocalUserId,
    pub command: Command,
}
