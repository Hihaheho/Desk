use bevy_math::Vec2;

use crate::shape::Shape;

use super::{event::WidgetEvent, Widget};

#[derive(Debug, Clone)]
pub struct RenderResponse<T: Iterator<Item = WidgetEvent>> {
    pub shape: Shape,
    pub events: T,
    pub drag_delta: Vec2,
}

pub trait WidgetBackend {
    type EventIterator: Iterator<Item = WidgetEvent>;

    fn render(&mut self, widget: &Widget) -> RenderResponse<Self::EventIterator>;
}
