use bevy::prelude::*;
use physics::{
    shape::Shape,
    widget::{component::Component, event::WidgetEvents},
    DragState, Velocity,
};

#[derive(Bundle)]
pub(crate) struct WidgetBundle {
    shape: Shape,
    component: Component,
    drag_state: DragState,
    velocity: Velocity,
    events: WidgetEvents,
}

impl Default for WidgetBundle {
    fn default() -> Self {
        Self {
            shape: Default::default(),
            component: Default::default(),
            drag_state: Default::default(),
            velocity: Default::default(),
            events: Default::default(),
        }
    }
}
