use bevy_ecs::prelude::Component;

#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub enum DragState {
    Dragging,
    NotDragging,
}

impl Default for DragState {
    fn default() -> Self {
        DragState::NotDragging
    }
}
