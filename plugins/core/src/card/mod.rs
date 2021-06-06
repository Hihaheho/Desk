use bevy::prelude::*;
use editor::card::{Card, Computed};
use language::code::node::{sugar, Node};
use systems::card::{create_card, render_card};

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(create_card_system.system())
            .add_system(card_rendering.system());
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
            card: create_card(),
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
        node: sugar::string(""),
        transform: Transform::from_xyz(100.0, 200.0, 0.0),
        ..Default::default()
    });
}

fn card_rendering(
    mut commands: Commands,
    query: Query<
        (Entity, &Card, &Node, Option<&Computed>, &Transform),
        Or<(
            Changed<Card>,
            Changed<Node>,
            Changed<Computed>,
            Changed<Transform>,
        )>,
    >,
) {
    for (entity, card, node, computed, transform) in query.iter() {
        if let Some(widget) = render_card(card, node, computed, transform.translation.into()) {
            commands.entity(entity).insert(widget);
        }
    }
}
