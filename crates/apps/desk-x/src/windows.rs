use bevy::prelude::*;

use desk_window::window::{DefaultWindow, Window};
use dworkspace::{
    prelude::{EventId, EventPayload},
    Workspace,
};
use dworkspace_codebase::{event::Event, user::UserId};
use dworkspace_in_memory::InMemoryRepository;

pub struct WindowsPlugin;

impl Plugin for WindowsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

pub fn setup(mut commands: Commands) {
    let user_id = UserId::new();
    let mut kernel = Workspace::new(InMemoryRepository::new(user_id));
    kernel.commit(Event {
        id: EventId::new(),
        user_id,
        payload: EventPayload::AddOwner { user_id },
    });
    commands.spawn((DefaultWindow, Window::<egui::Context>::default(), kernel));
}
