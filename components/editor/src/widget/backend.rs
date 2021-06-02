use super::{operation::WidgetOperation, shape::Shape, Vec2, Widget};

pub struct RenderResponse<T: Iterator<Item = WidgetOperation>> {
    pub position: Vec2,
    pub velocity: Vec2,
    pub shape: Shape,
    pub operations: T,
}

pub trait WidgetBackend {
    type OperationIterator: Iterator<Item = WidgetOperation>;

    fn render(&mut self, widget: &Widget) -> RenderResponse<Self::OperationIterator>;
}
