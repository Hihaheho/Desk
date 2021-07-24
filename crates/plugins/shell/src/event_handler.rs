use bevy::{ecs::schedule::ParallelSystemDescriptor, prelude::*};
use derivative::*;

use physics::event_handler::EventHandler;
use plugin_core::EventHandlerSystem;

#[derive(Derivative)]
#[derivative(Default(bound = ""))]
pub struct EventHandlerWrapper<T>(std::marker::PhantomData<fn() -> T>);

pub trait IntoEventHandlerSystem {
    fn event_handler_system(self) -> ParallelSystemDescriptor;
}

impl<T: EventHandler> IntoEventHandlerSystem for EventHandlerWrapper<T> {
    fn event_handler_system(self) -> ParallelSystemDescriptor {
        handle_widget_event::<T>
            .system()
            .label(EventHandlerSystem::<T>::Handle)
    }
}

fn handle_widget_event<Handler: EventHandler>(
    mut query: Query<(
        &Handler::Context,
        &Handler,
        &Handler::Events,
        &mut Handler::Output,
    )>,
) {
    for (context, handler, events, mut output) in query.iter_mut() {
        *output = handler.handle(context, events)
    }
}
