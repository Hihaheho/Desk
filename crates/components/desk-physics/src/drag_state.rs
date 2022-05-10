#[derive(Clone, Debug, PartialEq)]
pub enum DragState {
    Dragging,
    NotDragging,
}

impl Default for DragState {
    fn default() -> Self {
        DragState::NotDragging
    }
}
