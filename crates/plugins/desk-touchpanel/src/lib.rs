mod cursor_systems;
mod drag_system;

use system_ordering::{DeskSystem, ShellSystem};

use bevy::prelude::*;

pub struct ShellPlugin;

impl Plugin for ShellPlugin {
    fn build(&self, app: &mut bevy::app::AppBuilder) {
        app.init_resource::<Backends>()
            .add_startup_system(add_cursor.system())
            .add_system(
                move_cursor
                    .system()
                    .label(DeskSystem::UpdateStatesToLatest)
                    .before(DeskSystem::Shell),
            )
            .add_system(
                drag_system::toggle_follow_for_drag_state
                    .system()
                    .after(ShellSystem::Render)
                    .before(ShellSystem::HandleEvents),
            )
            .add_system(
                follow_system::follow
                    .system()
                    .after(DeskSystem::Shell)
                    .before(DeskSystem::PrePhysics),
            )
            .add_system(reset_velocity.system().after(DeskSystem::PrePhysics))
    }
}

fn reset_velocity(mut query: Query<&mut Velocity>) {
    for mut velocity in query.iter_mut() {
        velocity.0 = Vec2::ZERO;
    }
}
