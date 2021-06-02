use bevy::prelude::*;
use editor::card::{Card, Computed};
use heron::prelude::*;
use language::abstract_syntax_tree::node::{sugar, Node};
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
    rigid_body: RigidBody,
    collision_shape: CollisionShape,
    rotation_constraints: RotationConstraints,
    physic_material: PhysicMaterial,
}

impl Default for CardBundle {
    fn default() -> Self {
        CardBundle {
            card: create_card(),
            node: sugar::string(""),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            rigid_body: RigidBody::Dynamic,
            collision_shape: CollisionShape::Cuboid {
                half_extends: Vec3::new(1.0, 1.0, 0.0),
            },
            rotation_constraints: RotationConstraints::lock(),
            physic_material: PhysicMaterial::default(),
        }
    }
}

fn create_card_system(mut commands: Commands) {
    commands
        .spawn_bundle(CardBundle {
            node: sugar::add(sugar::integer(1), sugar::integer(2)),
            ..Default::default()
        })
        .insert(Velocity::default());

    commands
        .spawn_bundle(CardBundle {
            node: sugar::integer(1),
            ..Default::default()
        })
        .insert(Velocity::default());

    commands
        .spawn_bundle(CardBundle {
            node: sugar::string(""),
            ..Default::default()
        })
        .insert(Velocity::default());
}

fn card_rendering(
    mut commands: Commands,
    query: Query<(Entity, &Card, &Node, Option<&Computed>, &Transform)>,
) {
    for (entity, card, node, computed, transform) in query.iter() {
        if let Some(widget) = render_card(card, node, computed, transform.translation.into()) {
            commands.entity(entity).insert(widget);
        }
    }
}
