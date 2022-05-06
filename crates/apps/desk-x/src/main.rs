use bevy::prelude::*;
use egui_plugin::EguiPlugin;

#[cfg(feature = "inspector")]
use bevy_inspector_egui::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.0, 120.0 / 255.0, 120.0 / 255.0)))
        .add_startup_system(setup);

    #[cfg(feature = "inspector")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(PerspectiveCameraBundle::new_3d());
}
