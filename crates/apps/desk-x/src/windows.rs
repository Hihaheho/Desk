use bevy::prelude::*;

use desk_window::window::{Window, DefaultWindow};
use dkernel::Kernel;
use dkernel_components::user::UserId;
use dkernel_in_memory::InMemoryRepository;

pub struct WindowsPlugin;

impl Plugin for WindowsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

pub fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert(DefaultWindow)
        .insert(Window::<egui::Context>::default())
        .insert(Kernel::new(InMemoryRepository::new(UserId("me".into()))));
}
