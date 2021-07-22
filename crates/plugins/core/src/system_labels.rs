use bevy::prelude::*;
use derivative::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum DeskSystem {
    Shell,
    HandleOperations,
    PrePhysics,
}

#[derive(SystemLabel, PartialEq, Eq, Debug, Hash, Clone)]
pub enum ShellSystem {
    Add,
    UpdateComponent,
    UpdateWidget,
    Render,
    HandleEvents,
}

#[derive(Derivative)]
#[derivative(
    PartialEq(bound = ""),
    Eq(bound = ""),
    Debug(bound = ""),
    Hash(bound = ""),
    Clone(bound = "")
)]
pub enum EventHandlerSystem<T> {
    Before,
    Handle,
    After,
    _Phantom(std::convert::Infallible, std::marker::PhantomData<T>),
}

impl<T: Send + Sync + 'static> SystemLabel for EventHandlerSystem<T> {
    fn dyn_clone(&self) -> Box<dyn SystemLabel> {
        Box::new(self.clone())
    }
}
