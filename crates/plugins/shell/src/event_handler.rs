use bevy::prelude::*;
use derivative::*;

use physics::event_handler::EventHandler;
use plugin_core::EventHandlerSystem;

#[derive(Derivative)]
#[derivative(Default(bound = ""))]
pub struct EventHandlerPlugin<T>(std::marker::PhantomData<fn() -> T>);

impl<T: EventHandler> EventHandlerPlugin<T> {
    pub fn system(self) -> impl ParallelSystemDescriptorCoercion {
        handle_widget_event::<T>
            .system()
            .label(EventHandlerSystem::<T>::Handle)
    }
}

fn handle_widget_event<Handler: EventHandler>(
    mut commands: Commands,
    mut query: Query<(Entity, &Handler::Context, &Handler, &Handler::Events)>,
) {
    for (entity, context, handler, events) in query.iter_mut() {
        let output = handler.handle(context, events);
        commands.entity(entity).insert(output);
    }
}
