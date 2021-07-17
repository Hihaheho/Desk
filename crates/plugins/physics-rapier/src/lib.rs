use core::DeskSystem;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use physics::{
    shape::Shape,
    widget::{WidgetId},
    Velocity,
};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut bevy::app::AppBuilder) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(RapierRenderPlugin)
            .add_startup_system(walls.system())
            .insert_resource(RapierConfiguration {
                scale: 100.0,
                gravity: Vec2::ZERO.into(),
                ..Default::default()
            })
            .add_system(
                add_physics_components
                    .system()
                    .after(DeskSystem::Shell)
                    .before(DeskSystem::PrePhysics),
            )
            .add_system_set(
                SystemSet::new()
                    .label(DeskSystem::PrePhysics)
                    .with_system(update_shape.system())
                    .with_system(update_velocity.system()),
            );
    }
}

fn walls(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform.translation.x = 630.0;
    camera.transform.translation.y = 350.0;
    commands.spawn_bundle(LightBundle {
        light: Light {
            intensity: 100_000.0,
            range: 6000.0,
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn_bundle(camera);
    commands
        .spawn_bundle(ColliderBundle {
            position: Vec2::new(0.0, 0.0).into(),
            shape: ColliderShape::cuboid(0.1, 9.0),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::default());
    commands
        .spawn_bundle(ColliderBundle {
            position: Vec2::new(10.0, 0.0).into(),
            shape: ColliderShape::cuboid(0.1, 9.0),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::default());
    commands
        .spawn_bundle(ColliderBundle {
            position: Vec2::new(0.0, 0.0).into(),
            shape: ColliderShape::cuboid(12.0, 0.1),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::default());
    commands
        .spawn_bundle(ColliderBundle {
            position: Vec2::new(0.0, 7.0).into(),
            shape: ColliderShape::cuboid(12.0, 0.1),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::default());
}

fn add_physics_components(
    rapier: Res<RapierConfiguration>,
    mut commands: Commands,
    query: Query<(Entity, &GlobalTransform), Added<WidgetId>>,
) {
    for (card, transform) in query.iter() {
        commands
            .entity(card)
            .insert_bundle(RigidBodyBundle {
                position: (transform.translation / rapier.scale).into(),
                mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
                damping: RigidBodyDamping {
                    linear_damping: 2.0,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(RigidBodyPositionSync::Discrete)
            .with_children(|build| {
                build.spawn_bundle(ColliderBundle {
                    shape: ColliderShape::cuboid(0.1, 0.1),
                    ..Default::default()
                });
            });
    }
}

fn update_shape(
    rapier: Res<RapierConfiguration>,
    shape: Query<(&Shape, Entity, &Children)>,
    mut collider: Query<(&mut ColliderShape, &mut ColliderParent)>,
) {
    for (shape, entity, children) in shape.iter() {
        std::iter::once(&entity)
            .to_owned()
            .chain(children.iter())
            .for_each(|&entity| {
                if let Ok((mut collider_shape, mut parent)) = collider.get_mut(entity) {
                    use Shape::*;
                    match shape {
                        Rect { width, height } => {
                            let width = *width / rapier.scale / 2.0;
                            let height = *height / rapier.scale / 2.0;
                            *collider_shape = ColliderShape::cuboid(width, height);
                            parent.pos_wrt_parent.translation = Vec2::new(width, -height).into();
                        }
                        Blank => {}
                        _ => todo!(),
                    };
                }
            });
    }
}

fn update_velocity(
    rapier: Res<RapierConfiguration>,
    mut query: Query<(&mut RigidBodyVelocity, &Velocity), Changed<Velocity>>,
) {
    for (mut rapier_velocity, velocity) in query.iter_mut() {
        rapier_velocity.linvel.x = velocity.0.x / rapier.scale;
        rapier_velocity.linvel.y = velocity.0.y / rapier.scale;
    }
}
