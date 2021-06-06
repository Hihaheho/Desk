use std::ops::RangeInclusive;

use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Shape, Slider},
    EguiContext, EguiPlugin,
};
use editor::{
    card::Card,
    physics::Velocity,
    widget::{backend::WidgetBackend, Widget},
};
use egui_backend::EguiBackend;

struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut bevy::app::AppBuilder) {
        app.add_system(show_card.system());
    }
}

fn show_card(
    mut commands: Commands,
    egui_context: ResMut<EguiContext>,
    time: Res<Time>,
    mut query: Query<(Entity, &Widget)>,
) {
    for (entity, widget) in query.iter_mut() {
        let mut backend = EguiBackend {
            ctx: egui_context.ctx(),
            delta_seconds: time.delta_seconds(),
        };

        let response = backend.render(widget);
        commands
            .entity(entity)
            .insert(response.shape)
            .insert(response.velocity);
    }
}

pub struct EguiPlugins;

impl PluginGroup for EguiPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(CardPlugin).add(EguiPlugin);
    }
}
