use crate::{shape::Shape, Velocity};

use super::{operation::WidgetOperation, Vec2, Widget};

#[derive(Debug, Clone)]
pub struct RenderResponse<T: Iterator<Item = WidgetOperation>> {
    pub position: Vec2,
    pub velocity: Velocity,
    pub shape: Shape,
    pub operations: T,
}

pub trait WidgetBackend {
    type OperationIterator: Iterator<Item = WidgetOperation>;

    fn render(&mut self, widget: &Widget) -> RenderResponse<Self::OperationIterator>;
}
