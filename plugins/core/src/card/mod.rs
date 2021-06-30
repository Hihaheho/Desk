use bevy::prelude::*;
use language::code::node::{sugar, Node};
use shell::card::Card;

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(create_card_system.system());
    }
}

#[derive(Bundle)]
struct CardBundle {
    card: Card,
    node: Node,
    transform: Transform,
    global_transform: GlobalTransform,
}

impl Default for CardBundle {
    fn default() -> Self {
        CardBundle {
            card: Card::new(),
            node: sugar::string(""),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}

fn create_card_system(mut commands: Commands) {
    commands.spawn_bundle(CardBundle {
        node: sugar::add(sugar::integer(1), sugar::integer(2)),
        transform: Transform::from_xyz(100.0, 10.0, 0.0),
        ..Default::default()
    });

    commands.spawn_bundle(CardBundle {
        node: sugar::integer(1),
        transform: Transform::from_xyz(200.0, 100.0, 0.0),
        ..Default::default()
    });

    commands.spawn_bundle(CardBundle {
        node: sugar::string("aaaa"),
        transform: Transform::from_xyz(100.0, 200.0, 0.0),
        ..Default::default()
    });
}
