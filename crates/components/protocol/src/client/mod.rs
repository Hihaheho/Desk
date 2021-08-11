use std::borrow::Cow;
use std::collections::HashMap;

use futures::Sink;

use crate::{Command, Event};

pub trait Client {
    fn sender(&self) -> Box<dyn Sink<Command, Error = String> + Send + Sync + Unpin + 'static>;
    fn poll_once(&mut self) -> Option<Vec<Event>>;
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct ClientName(pub Cow<'static, str>);

pub type BoxClient = Box<dyn Client + Send + Sync + 'static>;

#[derive(Default)]
pub struct Clients {
    pub map: HashMap<ClientName, BoxClient>,
}

impl Clients {
    pub fn insert(&mut self, name: ClientName, client: BoxClient) {
        self.map.insert(name, client);
    }

    pub fn get_mut(&mut self, name: &ClientName) -> Option<&mut BoxClient> {
        self.map.get_mut(name)
    }
}
