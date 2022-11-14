use bevy::prelude::*;

use desk_window::window::{DefaultWindow, Window};
use dworkspace::Workspace;
use dworkspace_codebase::{event::Event, user::UserId};
use dworkspace_in_memory::InMemoryRepository;

pub struct WindowsPlugin;

impl Plugin for WindowsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

pub fn setup(mut commands: Commands) {
    let user_id = UserId("me".into());
    let mut kernel = Workspace::new(InMemoryRepository::new(user_id.clone()));
    kernel.commit(Event::AddOwner { user_id });
    commands.spawn((DefaultWindow, Window::<egui::Context>::default(), kernel));
}
