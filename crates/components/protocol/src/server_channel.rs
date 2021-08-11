use futures::prelude::*;
use tracing::debug;

use crate::before_login::BeforeLogin;
use crate::server::ServerState;
use crate::ServerInput;
use crate::{Command, Event, ServerContext, ServerStateDispatcher, UserAuthenticationHandler};

pub struct Channel {}

impl Channel {
    pub async fn connect(
        self,
        auth: Box<dyn UserAuthenticationHandler + Send + Sync>,
        command_stream: impl Stream<Item = Command> + Send + Unpin + 'static,
        event_sender: impl Sink<Event> + Send + Sync + Unpin,
    ) {
        let mut context = ServerContext {
            user_authentication_handler: auth,
            event_sender,
        };
        let state: ServerStateDispatcher = BeforeLogin {}.into();

        let mut stream = command_stream.map(|command| ServerInput::Command { command });

        while let Some(input) = stream.next().await {
            debug!("{:?}", input);
            state.handle(&mut context, &input).await;
        }
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
