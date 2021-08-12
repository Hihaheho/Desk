use std::borrow::Cow;

use futures::Sink;

use crate::{Command, Event};

pub trait Client {
    fn sender(&self) -> Box<dyn Sink<Command, Error = String> + Send + Sync + Unpin + 'static>;
    fn poll_once(&mut self) -> Option<Vec<Event>>;
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct ClientName(pub Cow<'static, str>);

pub type BoxClient = Box<dyn Client + Send + Sync + 'static>;
