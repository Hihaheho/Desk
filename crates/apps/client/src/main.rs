use core::{card::CardPlugin, language::LanguagePlugins};
use physics_rapier::PhysicsPlugin;

use bevy::prelude::*;
use shell::ShellPlugin;

#[bevy_main]
fn main() {
    let mut app = App::build();

    app.add_plugins(DefaultPlugins)
        .add_plugin(CardPlugin)
        .add_plugin(ShellPlugin)
        .add_plugins(LanguagePlugins)
        .add_plugin(PhysicsPlugin)
        .add_plugin(egui::EguiBackendPlugin);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(wasm_target::WasmTargetPlugin);

    #[cfg(feature = "bevy_mod_debugdump")]
    std::fs::write(
        "target/schedule_graph.dot",
        bevy_mod_debugdump::schedule_graph::schedule_graph_dot(&app.app.schedule),
    );

    app.run();
}
