use super::component::{InputId, Integer};

#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum WidgetEvent {
    UpdateString { id: InputId, value: String },
    UpdateInteger { id: InputId, value: Integer },
    LostFocus { id: InputId },
}
