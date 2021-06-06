use core::{card::CardPlugin, language::LanguagePlugins};
use physics_heron::PhysicsPlugin;

use bevy::prelude::*;

#[bevy_main]
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(CardPlugin)
        .add_plugins(LanguagePlugins)
        .add_plugin(PhysicsPlugin)
        .add_plugins(egui::EguiPlugins)
        .run();
}
