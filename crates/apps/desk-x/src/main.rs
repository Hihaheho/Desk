mod about;
mod windows;
use about::AboutPlugin;
use bevy::prelude::*;

#[cfg(feature = "inspector")]
use bevy_inspector_egui::WorldInspectorPlugin;
use editor_plugin::EditorPlugin;
use egui_plugin::EguiPlugin;
use rapier2d_plugin::PhysicsPlugin;
use terminal_plugin::TerminalPlugin;
use touchpanel_plugin::TouchpanelPlugin;
use windows::WindowsPlugin;

// #[cfg(target_arch = "wasm32")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(TouchpanelPlugin)
        .add_plugin(EditorPlugin)
        .add_plugin(TerminalPlugin)
        .add_plugin(WindowsPlugin)
        .add_plugin(AboutPlugin)
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.0, 120.0 / 255.0, 120.0 / 255.0)))
        .add_startup_system(setup_cameras);

    #[cfg(target_arch = "wasm32")]
    app.add_system(resize);

    #[cfg(feature = "inspector")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.run();
}

fn setup_cameras(mut commands: Commands) {
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
