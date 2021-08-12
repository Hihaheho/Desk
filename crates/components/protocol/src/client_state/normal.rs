use crate::{ClientContext, ClientInput, ClientState, ClientStateDispatcher, UserId};

#[derive(Debug, PartialEq, Clone)]
pub struct Normal {
    pub user_id: UserId,
}

impl ClientState for Normal {
    fn handle(&self, _commands: &mut ClientContext, _event: &ClientInput) -> ClientStateDispatcher {
        self.clone().into()
    }
}
