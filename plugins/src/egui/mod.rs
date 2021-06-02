use std::ops::RangeInclusive;

use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Slider},
    EguiContext, EguiPlugin,
};
use editor::{
    card::{Card, Computed},
    widget::Widget,
};
use heron::prelude::*;
use language::abstract_syntax_tree::node::{LiteralValue, Node, NodeData, NumberLiteral};

struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut bevy::app::AppBuilder) {
        app.add_system(show_card.system());
    }
}

fn show_card(
    egui_context: ResMut<EguiContext>,
    time: Res<Time>,
    mut query: Query<(
        &Card,
        &Transform,
        &mut Velocity,
        &mut CollisionShape,
        &Widget,
    )>,
) {
    let ctx = egui_context.ctx();
    for (card, transform, mut velocity, mut shape, widget) in query.iter_mut() {}
}

pub struct EguiPlugins;

impl PluginGroup for EguiPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(EguiPlugin).add(CardPlugin);
    }
}
