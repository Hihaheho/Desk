use std::ops::RangeInclusive;

use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Slider},
    EguiContext, EguiPlugin,
};
use editor::{
    card::{Card, Computed},
    widget::{backend::WidgetBackend, shape::Shape, Widget},
};
use egui_backend::EguiBackend;
use heron::prelude::*;
use language::code::node::{LiteralValue, Node, NodeData, NumberLiteral};

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
    for (card, transform, mut velocity, mut shape, widget) in query.iter_mut() {
        let mut backend = EguiBackend {
            ctx: egui_context.ctx(),
            delta_seconds: time.delta_seconds(),
        };

        let response = backend.render(widget);
        *velocity = response.velocity.into();
        *shape = match response.shape {
            Shape::Rect { width, height } => CollisionShape::Cuboid {
                half_extends: Vec3::new(width, height, 0.0),
            },
            _ => todo!(),
        }
    }
}

pub struct EguiPlugins;

impl PluginGroup for EguiPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(EguiPlugin).add(CardPlugin);
    }
}
