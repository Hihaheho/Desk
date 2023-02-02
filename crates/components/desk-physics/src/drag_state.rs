use bevy_ecs::prelude::Component;

#[derive(Default, Component, Clone, Debug, PartialEq, Eq)]
pub enum DragState {
    Dragging,
    #[default]
    NotDragging,
}
