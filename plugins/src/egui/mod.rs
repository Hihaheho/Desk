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
    for (card, transform, mut velocity, mut shape, widget) in query.iter_mut() {
        let pos = transform.translation;

        let card_widget = egui::Area::new(card.card_id.clone())
            .movable(true)
            .current_pos(egui::pos2(pos.x, pos.y))
            .show(ctx, |ui| {
                ui.label("card");
                match widget {
                    Widget::InputNumber { value, target } => match value {
                        NumberLiteral::Float(value) => {
                            let mut value = value.to_owned();
                            ui.add(Slider::new(&mut value, 0.0..=10.0));
                        }
                        NumberLiteral::Integer(value) => {
                            let mut value = value.to_owned();
                            ui.add(Slider::new(&mut value, 0..=10));
                        }
                        NumberLiteral::Rational(_, _) => {
                            todo!()
                        }
                    },
                    Widget::Unit => {}
                    Widget::InputString { value, target } => {}
                };
            });

        let width = card_widget.rect.width();
        let height = card_widget.rect.height();
        *shape = CollisionShape::Cuboid {
            half_extends: Vec3::new(width, height, 0.0),
        };
        let delta = card_widget.drag_delta();
        // TODO use systems.
        velocity.linear.x = delta.x / time.delta_seconds();
        velocity.linear.y = delta.y / time.delta_seconds();
    }
}

pub struct EguiPlugins;

impl PluginGroup for EguiPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(EguiPlugin).add(CardPlugin);
    }
}
