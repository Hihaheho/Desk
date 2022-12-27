mod cursor_systems;
mod drag_system;

use cursor_systems::{add_cursor, move_cursor};
use desk_plugin::{DeskSystem, ShellSystem};
use physics::Velocity;

use bevy::prelude::*;

pub struct TouchpanelPlugin;

impl Plugin for TouchpanelPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_startup_system(add_cursor)
            .add_system(move_cursor.before(DeskSystem::UpdateWidget))
            .add_system(
                drag_system::toggle_follow_for_drag_state
                    .after(ShellSystem::Render)
                    .before(ShellSystem::HandleEvents),
            )
            .add_system(reset_velocity.after(DeskSystem::PrePhysics));
    }
}

fn reset_velocity(mut query: Query<&mut Velocity>) {
    for mut velocity in query.iter_mut() {
        velocity.0 = Vec2::ZERO;
    }
}
