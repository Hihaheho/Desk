mod before_login;
mod normal;
pub(in crate::client_state) use before_login::*;
use enum_dispatch::enum_dispatch;
pub(in crate::client_state) use normal::*;

use crate::{Commands, Event};

#[enum_dispatch(ClientState)]
#[derive(Debug, PartialEq, Clone)]
pub enum ClientStateDispatcher {
    BeforeLogin,
    Normal,
}

impl Default for ClientStateDispatcher {
    fn default() -> Self {
        Self::BeforeLogin(BeforeLogin {})
    }
}

#[enum_dispatch]
pub trait ClientState {
    fn handle(&self, commands: &mut ClientContext, event: &ClientInput) -> ClientStateDispatcher;
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct ClientContext {
    pub commands: Commands,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ClientInput {
    Event(Event),
}
