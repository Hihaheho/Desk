use plugin_language::LanguagePlugin;
use plugin_physics_rapier::PhysicsPlugin;

use bevy::prelude::*;
use plugin_egui::EguiBackendPlugin;
use plugin_shell::ShellPlugin;

#[bevy_main]
fn main() {
    let mut app = App::build();

    app.add_plugins(DefaultPlugins)
        .add_plugin(ShellPlugin)
        .add_plugin(LanguagePlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(EguiBackendPlugin);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(plugin_wasm_target::WasmTargetPlugin);

    #[cfg(feature = "bevy_mod_debugdump")]
    {
        std::fs::write(
            "target/schedule_graph.dot",
            bevy_mod_debugdump::schedule_graph::schedule_graph_dot(&app.app.schedule),
        );
        std::process::exit(0);
    }

    app.run();
}
