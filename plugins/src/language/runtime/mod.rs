use std::collections::HashMap;

use bevy::prelude::*;
use protocol::id::create_consistent_id;

pub struct RuntimePlugin;

impl Plugin for RuntimePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(run.system());
    }
}

fn run(mut commands: Commands, query: Query<(Entity, &IR), Changed<IR>>) {
    for (entity, code) in query.iter() {
        commands
            .entity(entity)
            .insert(prototype::compute_on_stack(code));
    }
}
