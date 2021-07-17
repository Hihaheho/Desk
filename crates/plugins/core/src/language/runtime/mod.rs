use bevy::prelude::*;
use language::code::node::Node;
use runtime::{card::Computed, Runtime};
use simple_traverse_runtime::SimpleTraverseRuntime;

pub struct RuntimePlugin;

impl Plugin for RuntimePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(run.system());
    }
}

fn run(mut commands: Commands, query: Query<(Entity, &Node), Changed<Node>>) {
    let runtime = SimpleTraverseRuntime;
    for (entity, code) in query.iter() {
        commands
            .entity(entity)
            .insert(Computed(runtime.run(code).unwrap()));
    }
}
