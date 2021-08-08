#![cfg(target_arch = "wasm32")]
use bevy::prelude::*;

pub struct WasmTargetPlugin;

impl Plugin for WasmTargetPlugin {
    fn build(&self, app: &mut bevy::app::AppBuilder) {
        app.add_plugin(bevy_webgl2::WebGL2Plugin)
            .add_plugin(bevy_kira_audio::AudioPlugin)
            .add_system(resize.system());
    }
}

fn resize(mut windows: ResMut<Windows>) {
    let js_window = web_sys::window().unwrap();
    let window = windows.get_primary_mut().unwrap();
    window.set_resolution(
        js_window.inner_width().unwrap().as_f64().unwrap() as f32,
        js_window.inner_height().unwrap().as_f64().unwrap() as f32,
    );
}
