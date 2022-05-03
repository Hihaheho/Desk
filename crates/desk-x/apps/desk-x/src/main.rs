use bevy::prelude::*;
use egui_plugin::EguiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .insert_resource(Msaa { samples: 4 })
        .run();
}
