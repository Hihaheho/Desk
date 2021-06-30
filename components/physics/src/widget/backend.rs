use crate::{shape::Shape, Velocity};

use super::{event::WidgetEvent, Vec2, Widget};

#[derive(Debug, Clone)]
pub struct RenderResponse<T: Iterator<Item = WidgetEvent>> {
    pub position: Vec2,
    pub velocity: Velocity,
    pub shape: Shape,
    pub events: T,
}

pub trait WidgetBackend {
    type OperationIterator: Iterator<Item = WidgetEvent>;

    fn render(&mut self, widget: &Widget) -> RenderResponse<Self::OperationIterator>;
}
