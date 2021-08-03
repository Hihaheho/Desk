use std::{
    any::{Any, TypeId},
    collections::HashMap,
    convert::TryInto,
    future::Future,
    thread::sleep,
    time::Duration,
};

use cmd_lib::{run_cmd, run_fun, spawn};
use httpmock::MockServer;
use hyper::{Client, Uri};
use portpicker::pick_unused_port;
use std::cell::RefCell;
use std::rc::Rc;

mod http_request;

#[derive(Default)]
pub struct ContextBuilder {
    before: Vec<fn(&mut Context) -> ()>,
    after: Vec<fn(&mut Context) -> ()>,
}

impl ContextBuilder {
    pub fn before(mut self, function: fn(&mut Context) -> ()) -> Self {
        self.before.push(function);
        self
    }

    pub fn after(mut self, function: fn(&mut Context) -> ()) -> Self {
        self.after.push(function);
        self
    }

    pub fn build(self) -> Context {
        let mut context = Context {
            map: Default::default(),
            after: self.after,
        };

        for before in self.before.iter() {
            before(&mut context);
        }

        context
    }
}

pub struct Context {
    map: HashMap<TypeId, Box<dyn Any>>,
    after: Vec<fn(&mut Context) -> ()>,
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

    pub fn after(&mut self, function: fn(&mut Context) -> ()) -> &mut Self {
        self.after.push(function);
        self
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

impl Drop for Context {
    fn drop(&mut self) {
        for after in self.after.clone() {
            after(self);
        }
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

pub struct ContainerId(pub String);

pub fn start_desk_server(ctx: &mut Context) {
    let port = pick_unused_port().unwrap();
    let container_id = run_fun! {
        docker run --rm -d -p $port:8080 -e PORT=8080 gcr.io/hihaheho/desk-server:latest;
    }
    .unwrap();
    let url = format!("http://localhost:{}", port);
    while run_cmd!(curl $url > /dev/null).is_err() {
        sleep(Duration::from_secs(1));
    }
    ctx.insert::<Uri>(url.try_into().unwrap());
    ctx.insert(ContainerId(container_id));
    ctx.after(stop_desk_server);
}

fn stop_desk_server(ctx: &mut Context) {
    let container_id = ctx.get::<ContainerId>().unwrap().0.clone();
    let _ = spawn!(
        docker stop $container_id > /dev/null;
    )
    .unwrap();
}

#[test]
fn before() {
    fn add(context: &mut Context) {
        context.insert::<u8>(42);
    }

    let context = builder().before(add).build();
    assert_eq!(context.get::<u8>(), Some(&42_u8));
}

fn call(context: &mut Context) {
    let called = context.get::<Rc<RefCell<bool>>>().unwrap().clone();
    *called.borrow_mut() = true;
}

#[test]
fn builder_after() {
    let called = Rc::new(RefCell::new(false));
    let mut context = builder().after(call).build();
    context.insert(called.clone());
    assert!(!*called.borrow());
    std::mem::drop(context);
    assert!(*called.borrow());
}

#[test]
fn context_after() {
    let called = Rc::new(RefCell::new(false));
    let mut context = builder().build();
    context.insert(called.clone());
    context.after(call);
    assert!(!*called.borrow());
    std::mem::drop(context);
    assert!(*called.borrow());
}
