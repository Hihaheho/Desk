use tracing::error;

use crate::{ClientContext, ClientInput, ClientState, ClientStateDispatcher, Event};

use super::normal::Normal;

#[derive(Debug, PartialEq, Clone)]
pub struct BeforeLogin {}

impl ClientState for BeforeLogin {
    fn handle(&self, _commands: &mut ClientContext, event: &ClientInput) -> ClientStateDispatcher {
        match event {
            ClientInput::Event(Event::LoggedIn { user_id }) => {
                return Normal {
                    user_id: user_id.clone(),
                }
                .into();
            }
            event => {
                error!("unexpected event: {:?}", event);
            }
        }
        self.clone().into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::client_state::normal::Normal;
    use crate::{ClientContext, UserId};

    #[test]
    fn handles_logged_in_event() {
        let state = BeforeLogin {};
        let mut context = ClientContext {
            ..Default::default()
        };
        let user_id = UserId::generate();
        let event = ClientInput::Event(Event::LoggedIn {
            user_id: user_id.clone(),
        });

        assert_eq!(
            state.handle(&mut context, &event),
            Normal { user_id }.into()
        );
    }
}
