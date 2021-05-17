pub mod runtime;

use bevy::prelude::*;

use self::runtime::RuntimePlugin;
pub struct LanguagePlugins;

impl PluginGroup for LanguagePlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(RuntimePlugin);
    }
}
