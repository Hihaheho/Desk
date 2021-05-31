use bevy::prelude::*;
use heron::Velocity;
use language::abstract_syntax_tree::node::{
    sugar, BinaryArithmeticOperator, BinaryOperator, Node, NodeData,
};
use systems::card::create_card;

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(create_card_system.system());
    }
}

fn create_card_system(mut commands: Commands) {
    commands
        .spawn()
        .insert(create_card())
        .insert(sugar::add(sugar::integer(1), sugar::integer(2)))
        .insert(Velocity::default())
        .insert(Transform::default())
        .insert(GlobalTransform::default());

    commands.spawn().insert(create_card());
}
