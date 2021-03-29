mod plugins;

use bevy::prelude::*;

#[bevy_main]
fn main() {
    App::build().add_plugins(DefaultPlugins).run();
}
