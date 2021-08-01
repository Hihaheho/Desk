use std::{
    any::{Any, TypeId},
    collections::HashMap,
    convert::TryInto,
    future::Future,
    thread::sleep,
    time::Duration,
};

use cmd_lib::{run_cmd, spawn};
use httpmock::MockServer;
use hyper::{Client, Uri};

mod http_request;

#[derive(Default)]
pub struct ContextBuilder {
    before: Vec<fn(&mut Context) -> ()>,
}

impl ContextBuilder {
    pub fn before(&mut self, function: fn(&mut Context) -> ()) -> &mut Self {
        self.before.push(function);
        self
    }

    pub fn build(&self) -> Context {
        let mut context = Context::default();

        for before in self.before.iter() {
            before(&mut context);
        }

        context
    }
}

#[derive(Default)]
pub struct Context {
    map: HashMap<TypeId, Box<dyn Any>>,
}

impl Context {
    pub fn insert<T: 'static>(&mut self, value: T) {
        self.map.insert(TypeId::of::<T>(), Box::new(value));
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.map
            .get(&TypeId::of::<T>())
            .and_then(|any| any.downcast_ref())
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.map
            .get_mut(&TypeId::of::<T>())
            .and_then(|any| any.downcast_mut())
    }

    pub fn _run(&mut self, function: impl FnOnce(&mut Self) -> &mut Self) -> &mut Self {
        function(self)
    }

    pub async fn run_async<'a, T: FnOnce(&'a mut Self) -> F, F: Future<Output = &'a mut Self>>(
        &'a mut self,
        function: T,
    ) -> &'a mut Self {
        function(self).await
    }
}

pub fn builder() -> ContextBuilder {
    Default::default()
}

pub fn http_client(ctx: &mut Context) {
    let client = Client::new();
    ctx.insert(client);
}

pub fn start_mock_server(ctx: &mut Context) {
    let server = MockServer::start();
    ctx.insert::<Uri>(server.base_url().try_into().unwrap());
    ctx.insert(server);
}

pub fn start_desk_server(ctx: &mut Context) {
    let server = spawn!(
        ../../../target/debug/desk-server -p 4000;
    )
    .unwrap();
    let url = "http://localhost:4000";
    while run_cmd!(curl $url).is_err() {
        sleep(Duration::from_secs(1));
    }
    ctx.insert::<Uri>(url.try_into().unwrap());
}

#[test]
fn before() {
    fn add(context: &mut Context) {
        context.insert::<u8>(42);
    }

    let context = builder().before(add).build();
    assert_eq!(context.get::<u8>(), Some(&42_u8));
}
