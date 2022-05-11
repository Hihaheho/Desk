use bevy::prelude::*;
use egui_plugin::EguiPlugin;

#[cfg(feature = "inspector")]
use bevy_inspector_egui::WorldInspectorPlugin;
use rapier2d_plugin::PhysicsPlugin;
use touchpanel_plugin::TouchpanelPlugin;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(TouchpanelPlugin)
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.0, 120.0 / 255.0, 120.0 / 255.0)))
        .add_startup_system(setup);

    #[cfg(target_arch = "wasm32")]
    app.add_system(resize);

    #[cfg(feature = "inspector")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(PerspectiveCameraBundle::new_3d());
}

#[cfg(target_arch = "wasm32")]
fn resize(mut windows: ResMut<Windows>) {
    let js_window = web_sys::window().unwrap();
    let window = windows.get_primary_mut().unwrap();
    window.set_resolution(
        js_window.inner_width().unwrap().as_f64().unwrap() as f32,
        js_window.inner_height().unwrap().as_f64().unwrap() as f32,
    );
}
