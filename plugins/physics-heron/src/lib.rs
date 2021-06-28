use bevy::prelude::*;
use heron::{CollisionShape, PhysicMaterial, RigidBody, RotationConstraints};
use physics::{Velocity, shape::Shape, widget::Widget};
use shell::card::Card;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut bevy::app::AppBuilder) {
        app.add_plugin(heron::prelude::PhysicsPlugin::default())
            .add_system(add_physics_components.system())
            .add_system(update_shape.system())
            .add_system(update_velocity.system());
    }
}

fn add_physics_components(mut commands: Commands, query: Query<Entity, Added<Card>>) {
    for card in query.iter() {
        commands
            .entity(card)
            .insert(heron::Velocity::default())
            .insert(RigidBody::Dynamic)
            .insert(CollisionShape::Cuboid {
                half_extends: Vec3::new(1.0, 1.0, 0.0),
            })
            .insert(RotationConstraints::lock())
            .insert(PhysicMaterial::default());
    }
}

fn update_shape(mut query: Query<(&mut CollisionShape, &Shape), Changed<Shape>>) {
    for (mut collision_shape, shape) in query.iter_mut() {
        use Shape::*;
        *collision_shape = match shape {
            Rect { width, height } => CollisionShape::Cuboid {
                half_extends: Vec3::new(*width, *height, 0.0),
            },
            _ => todo!(),
        };
    }
}

fn update_velocity(mut query: Query<(&mut heron::Velocity, &Velocity), Changed<Velocity>>) {
    for (mut heron_velocity, velocity) in query.iter_mut() {
        heron_velocity.linear.x = velocity.0.x;
        heron_velocity.linear.y = velocity.0.y;
    }
}
