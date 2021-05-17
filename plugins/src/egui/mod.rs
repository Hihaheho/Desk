use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use editor::card::Card;
use language::semantic::ir::IR;
use runtime::{ComputedValue, EncodedValue};

struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut bevy::app::AppBuilder) {
        app.add_system(ui_example.system())
            .add_system(show_card.system());
    }
}

fn show_card(
    egui_context: ResMut<EguiContext>,
    mut query: Query<(&mut Card, Option<&ComputedValue>, Option<&IR>)>,
) {
    let ctx = egui_context.ctx();
    for (mut card, computed_value, code) in query.iter_mut() {
        let pos = card.position;

        let card_widget = egui::Area::new(card.card_id.clone())
            .movable(true)
            .current_pos(egui::pos2(pos.x, pos.y))
            .show(ctx, |ui| {
                ui.label("card");
                if let Some(computed_value) = computed_value {
                    match computed_value.encoded_value {
                        EncodedValue::I32(value) => {
                            ui.label(format!("{:?}", value));
                        }
                        _ => {}
                    };
                }
            });

        let delta = card_widget.drag_delta();
        // TODO use systems.
        card.position += Vec2::new(delta.x, delta.y);
    }
}

fn ui_example(egui_context: ResMut<EguiContext>) {
    let ctx = egui_context.ctx();

    egui::Window::new("Hello").show(ctx, |ui| {
        ui.label("world");
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.add(egui::Label::new("Side Panel"));
    });

    egui::TopPanel::top("header").show(ctx, |ui| {
        ui.add(egui::Label::new("Top Panel"));
    });

    egui::SidePanel::left("left", 300.0).show(ctx, |ui| {
        ui.heading("Desk");

        ui.separator();

        ui.visuals();
    });

    egui::Area::new("first").show(ctx, |ui| {
        let _ = ui.button("click me");
    });
}

pub struct EguiPlugins;

impl PluginGroup for EguiPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(EguiPlugin).add(CardPlugin);
    }
}
