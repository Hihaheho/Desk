use core::{card::CardPlugin, language::LanguagePlugins, shell::ShellPlugin};
use physics_heron::PhysicsPlugin;

use bevy::prelude::*;

#[bevy_main]
fn main() {
    let mut app = App::build();

    app.add_plugins(DefaultPlugins)
        .add_plugin(CardPlugin)
        .add_plugin(ShellPlugin)
        .add_plugins(LanguagePlugins)
        .add_plugin(PhysicsPlugin)
        .add_plugins(egui::EguiPlugins);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(wasm_target::WasmTargetPlugin);

    app.run();
}
