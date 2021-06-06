use bevy_math::Vec2;
use editor::{physics::{Velocity, shape::Shape}, widget::{
        backend::{RenderResponse, WidgetBackend},
        operation::WidgetOperation,
        Component, Widget,
    }};
use language::code::node::NumberLiteral;

pub struct EguiBackend<'a> {
    pub ctx: &'a egui::CtxRef,
    pub delta_seconds: f32,
}

impl<'a> WidgetBackend for EguiBackend<'a> {
    type OperationIterator = std::vec::IntoIter<WidgetOperation>;

    fn render(&mut self, widget: &Widget) -> RenderResponse<Self::OperationIterator> {
        let mut operation_buffer = vec![];
        let card_widget = egui::Area::new(widget.id.as_str())
            .movable(true)
            .current_pos(egui::pos2(widget.position.x, widget.position.y))
            .show(self.ctx, |ui| {
                ui.label("card");

                use Component::*;
                match &widget.component {
                    InputNumber { value, target } => match value {
                        NumberLiteral::Float(value) => {
                            let mut value = value.to_owned();
                            ui.add(egui::Slider::new(&mut value, 0.0..=10.0));
                        }
                        NumberLiteral::Integer(value) => {
                            let mut value = value.to_owned();
                            ui.add(egui::Slider::new(&mut value, 0..=10));
                        }
                        NumberLiteral::Rational(_, _) => {
                            todo!()
                        }
                    },
                    Unit => {}
                    InputString { value, target } => {}
                };
            });

        let width = card_widget.rect.width();
        let height = card_widget.rect.height();
        let shape = Shape::Rect { width, height };
        let delta = card_widget.drag_delta();
        let velocity = Vec2::new(delta.x, delta.y) / self.delta_seconds;

        RenderResponse {
            position: widget.position,
            velocity: velocity.into(),
            shape,
            operations: operation_buffer.into_iter(),
        }
    }
}
