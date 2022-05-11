use bevy::prelude::*;

pub struct FileSystemPlugin;

impl Plugin for FileSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup() {}
