use super::component::{InputId, Integer};

#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum WidgetEvent {
    UpdateString { id: InputId, value: String },
    UpdateInteger { id: InputId, value: Integer },
    LostFocus { id: InputId },
}

#[derive(Clone, Debug, Default)]
pub struct WidgetEvents(pub Vec<WidgetEvent>);

impl WidgetEvents {
    pub fn iter(&self) -> impl Iterator<Item = &WidgetEvent> {
        self.0.iter()
    }
}

impl From<Vec<WidgetEvent>> for WidgetEvents {
    fn from(from: Vec<WidgetEvent>) -> Self {
        Self(from)
    }
}
