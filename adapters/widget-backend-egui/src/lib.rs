use bevy_math::Vec2;
use physics::{
    shape::Shape,
    widget::{
        backend::{RenderResponse, WidgetBackend},
        component::{Component, Orientation},
        event::WidgetEvent,
        Widget,
    },
};

pub use bevy_egui::egui;
use bevy_egui::egui::{CtxRef, Ui};
pub use bevy_egui::{EguiContext, EguiPlugin};

pub struct EguiBackend {
    pub ctx: CtxRef,
}

impl WidgetBackend for EguiBackend {
    type EventIterator = std::vec::IntoIter<WidgetEvent>;

    fn render(&mut self, widget: &Widget) -> RenderResponse<Self::EventIterator> {
        let mut event_buffer = vec![];
        if widget.component == Component::Blank {
            return RenderResponse {
                drag_delta: Vec2::ZERO.into(),
                shape: Shape::Blank,
                events: event_buffer.into_iter(),
            };
        }

        let card_widget = egui::Area::new(widget.id.to_owned())
            .current_pos(egui::pos2(
                widget.position.x,
                self.ctx.available_rect().height() - widget.position.y,
            ))
            .show(&self.ctx, |ui| {
                ui.label("====");
                render(ui, &mut event_buffer, &widget.component);
            });
        let width = card_widget.rect.width();
        let height = card_widget.rect.height();
        let shape = Shape::Rect { width, height };
        let delta = card_widget.drag_delta();
        let drag_delta = Vec2::new(delta.x, -delta.y);

        RenderResponse {
            drag_delta,
            shape,
            events: event_buffer.into_iter(),
        }
    }
}

fn render(ui: &mut Ui, event_buffer: &mut Vec<WidgetEvent>, component: &Component) {
    use Component::*;
    match component {
        InputInteger { id: _, value } => {
            let mut value = value.0;
            ui.add(egui::Slider::new(&mut value, 0..=10));
        }
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
