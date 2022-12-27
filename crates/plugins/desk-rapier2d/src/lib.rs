mod follow_system;

use desk_plugin::DeskSystem;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use physics::{shape::Shape, DragState, PhysicalObject};

pub struct PhysicsPlugin;

const LINEAR_DAMPING: f32 = 8.0;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugin(RapierDebugRenderPlugin::default())
            .insert_resource(RapierConfiguration {
                gravity: Vec2::ZERO,
                ..Default::default()
            })
            .add_system(
                add_physics_components
                    .after(DeskSystem::UpdateWidget)
                    .before(DeskSystem::PrePhysics),
            )
            .add_system(follow_system::follow.before(DeskSystem::PrePhysics))
            .add_system_set(
                SystemSet::new()
                    .label(DeskSystem::PrePhysics)
                    .with_system(update_shape)
                    .with_system(update_velocity)
                    .with_system(update_drag_state),
            );
    }
}

fn add_physics_components(mut commands: Commands, query: Query<Entity, Added<PhysicalObject>>) {
    for card in query.iter() {
        commands
            .entity(card)
            .insert(RigidBody::Dynamic)
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(Damping {
                linear_damping: LINEAR_DAMPING,
                ..Default::default()
            })
            .insert(Collider::cuboid(0.1, 0.1));
    }
}

fn update_shape(
    shape: Query<(&Shape, Entity, &Children)>,
    mut collider: Query<(&mut Collider, &mut Transform)>,
) {
    for (shape, entity, children) in shape.iter() {
        std::iter::once(&entity)
            .to_owned()
            .chain(children.iter())
            .for_each(|&entity| {
                if let Ok((mut collider_shape, mut relative)) = collider.get_mut(entity) {
                    use Shape::*;
                    match shape {
                        Rect { width, height } => {
                            let width = *width / 100.0 / 2.0;
                            let height = *height / 100.0 / 2.0;
                            *collider_shape = Collider::cuboid(width, height);
                            relative.translation = Vec2::new(width, -height).extend(0.0);
                        }
                        Blank => {}
                        _ => todo!(),
                    };
                }
            });
    }
}

fn update_velocity(mut query: Query<(&mut Velocity, &physics::Velocity), Changed<Velocity>>) {
    for (mut rb_velocity, velocity) in query.iter_mut() {
        rb_velocity.linvel.x = velocity.0.x / 100.0;
        rb_velocity.linvel.y = velocity.0.y / 100.0;
    }
}

fn update_drag_state(mut query: Query<(&mut Damping, &DragState), Changed<DragState>>) {
    for (mut damping, drag_state) in query.iter_mut() {
        use DragState::*;
        match drag_state {
            Dragging => {
                damping.linear_damping = 0.0;
            }
            NotDragging => {
                damping.linear_damping = LINEAR_DAMPING;
            }
        }
    }
}
