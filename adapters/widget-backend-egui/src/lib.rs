use bevy_math::Vec2;
use editor::widget::{
    backend::{RenderResponse, WidgetBackend},
    operation::WidgetOperation,
    Component, Widget,
};
use language::abstract_syntax_tree::node::NumberLiteral;

pub struct EguiBackend {
    operation_buffer: Vec<WidgetOperation>,
    ctx: egui::CtxRef,
    delta_time: f32,
}

impl WidgetBackend for EguiBackend {
    type OperationIterator = std::vec::IntoIter<WidgetOperation>;

    fn render(&mut self, widget: &Widget) -> RenderResponse<Self::OperationIterator> {
        let card_widget = egui::Area::new(widget.id.as_str())
            .movable(true)
            .current_pos(egui::pos2(widget.position.x, widget.position.y))
            .show(&self.ctx, |ui| {
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
        // *shape = CollisionShape::Cuboid {
        //     half_extends: Vec3::new(width, height, 0.0),
        // };
        let delta = card_widget.drag_delta();
        // TODO use systems.
        let velocity = Vec2::new(delta.x, delta.y) / self.delta_time;
        todo!()
    }
}