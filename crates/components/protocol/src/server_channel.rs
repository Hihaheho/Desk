use futures::channel::mpsc::channel;
use futures::prelude::*;
use futures::stream::select;

use crate::server_state::ServerState;
use crate::{Command, Event, ServerStateDispatcher, UserAuthenticationHandler};
use crate::{InboundEntranceCommand, ServerInput, SinkAndStreamServerContext};

pub struct Channel {}

impl Channel {
    pub async fn connect(
        self,
        auth: Box<dyn UserAuthenticationHandler + Send + Sync>,
        command_stream: impl Stream<Item = Command> + Send + Unpin + 'static,
        event_sender: impl Sink<Event> + Send + Sync + Unpin,
    ) {
        let (entrance_event_sender, inbound_rx) = channel::<InboundEntranceCommand>(32);
        let context = SinkAndStreamServerContext {
            user_authentication_handler: auth,
            event_sender,
            entrance_command_sender: entrance_event_sender,
        };
        let state: ServerStateDispatcher = Default::default();

        let command_stream = command_stream.map(ServerInput::Command);
        let inbound_stream = inbound_rx.map(ServerInput::InboundEntranceCommand);
        select(command_stream, inbound_stream)
            .fold((context, state), |(mut context, state), input| async move {
                let state = state.handle(&mut context, &input).await;
                (context, state)
            })
            .await;
    }
}

#[cfg(test)]
mod test {
    use crate::mock::MockUserAuthenticationHandler;
    use crate::{primitives::*, Login};

    use super::*;
    use futures::channel::mpsc;

    use uuid::Uuid;

    #[tokio::test]
    async fn login() {
        let auth = MockUserAuthenticationHandler::default();
        let (mut tx_command, rx_command) = mpsc::channel::<Command>(1);
        let (tx_event, mut rx_event) = mpsc::channel::<Event>(1);
        let channel = Channel {};
        tokio::spawn(channel.connect(Box::new(auth.clone()), rx_command, tx_event));

        let token: Token = vec![1, 2, 3].into();
        let user_id = UserId(Uuid::new_v4());

        auth.authenticate
            .given(token.clone())
            .will_return(Ok(user_id.clone()));

        tx_command
            .send(Command::Login(Login { token }))
            .await
            .unwrap();
        assert_eq!(rx_event.next().await, Some(Event::LoggedIn { user_id }));
    }
}
