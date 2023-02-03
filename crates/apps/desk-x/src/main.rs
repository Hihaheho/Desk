mod about;
mod panels;
mod windows;

use std::time::Duration;

use about::AboutPlugin;
use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    winit::{UpdateMode, WinitSettings},
};

#[cfg(feature = "inspector")]
use bevy_inspector_egui::WorldInspectorPlugin;
use editor_plugin::EditorPlugin;
use egui_plugin::EguiPlugin;
use panels::PanelsPlugin;
use playground_plugin::PlaygroundPlugin;
// use rapier2d_plugin::PhysicsPlugin;
use terminal_plugin::TerminalPlugin;
use touchpanel_plugin::TouchpanelPlugin;
use windows::WindowsPlugin;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        // .add_plugin(PhysicsPlugin)
        .add_plugin(TouchpanelPlugin)
        .add_plugin(EditorPlugin)
        .add_plugin(TerminalPlugin)
        .add_plugin(WindowsPlugin)
        .add_plugin(AboutPlugin)
        .add_plugin(PanelsPlugin)
        .add_plugin(PlaygroundPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .insert_resource(WinitSettings {
            focused_mode: UpdateMode::ReactiveLowPower {
                max_wait: Duration::from_millis(1000),
            },
            unfocused_mode: UpdateMode::ReactiveLowPower {
                max_wait: Duration::from_millis(2000),
            },
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_startup_system(setup_cameras);

    #[cfg(target_arch = "wasm32")]
    app.add_system(resize);

    #[cfg(feature = "inspector")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.run();
}

fn setup_cameras(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default());
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
