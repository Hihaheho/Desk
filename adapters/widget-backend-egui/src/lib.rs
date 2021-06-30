use bevy_math::Vec2;
use egui::Ui;
use language::code::node::NumberLiteral;
use physics::{
    shape::Shape,
    widget::{
        backend::{RenderResponse, WidgetBackend},
        component::{Component, Orientation},
        event,
        event::WidgetEvent,
        Widget,
    },
};

pub struct EguiBackend<'a> {
    pub ctx: &'a egui::CtxRef,
    pub delta_seconds: f32,
}

impl<'a> WidgetBackend for EguiBackend<'a> {
    type OperationIterator = std::vec::IntoIter<WidgetEvent>;

    fn render(&mut self, widget: &Widget) -> RenderResponse<Self::OperationIterator> {
        let mut event_buffer = vec![];
        let card_widget = egui::Area::new(&widget.id)
            .movable(true)
            .current_pos(egui::pos2(widget.position.x, widget.position.y))
            .show(self.ctx, |ui| {
                ui.label("card");

                render(ui, &mut event_buffer, &widget.component);
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
            events: event_buffer.into_iter(),
        }
    }
}

fn render(ui: &mut Ui, event_buffer: &mut Vec<WidgetEvent>, component: &Component) {
    use Component::*;
    match component {
        InputNumber { id, value } => match value {
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
        Blank => {}
        InputString { id, value } => {
            let mut value = value.clone();
            let response = ui.text_edit_singleline(&mut value);

            if response.changed() {
                let id = id.to_owned();
                event_buffer.push(WidgetEvent::UpdateString { id, value });
            }
            if response.lost_focus() {
                let id = id.to_owned();
                event_buffer.push(WidgetEvent::LostFocus { id });
            }
        }
        Array { orientation, items } => {
            match orientation {
                Orientation::Vertical => {
                    ui.vertical(|ui| items.iter().for_each(|item| render(ui, event_buffer, item)))
                }
                Orientation::Horizontal => {
                    ui.horizontal(|ui| items.iter().for_each(|item| render(ui, event_buffer, item)))
                }
            };
        }
        Label(value) => {
            ui.label(value);
        }
        _ => todo!(),
    };
}
