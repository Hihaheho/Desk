use core::DeskSystem;

use bevy::prelude::*;
use language::{
    code::{node::Code, operation::CodeOperations},
    Computed, RuntimeId, Runtimes,
};

pub struct LanguagePlugin;

impl Plugin for LanguagePlugin {
    fn build(&self, app: &mut bevy::app::AppBuilder) {
        app.init_resource::<Runtimes>()
            .add_system_set(
                SystemSet::new()
                    .label(DeskSystem::HandleOperations)
                    .after(DeskSystem::Shell)
                    .before(DeskSystem::PrePhysics)
                    .with_system(handle_operation.system()),
            )
            .add_system(run.system());
    }
}

fn handle_operation(mut query: Query<(&mut Code, &CodeOperations)>) {
    for (mut code, operations) in query.iter_mut() {
        for operation in operations.iter() {
            let applied = code.apply_operation(operation);
            if let Ok(applied) = applied {
                *code = applied;
            }
        }
    }
}

fn run(
    mut commands: Commands,
    mut runtimes: ResMut<Runtimes>,
    query: Query<(Entity, &Code, Option<&RuntimeId>), Changed<Code>>,
) {
    for (entity, code, runtime_id) in query.iter() {
        if let Some(runtime) = runtime_id.and_then(|id| runtimes.get_mut(id)).or(None) {
            commands
                .entity(entity)
                .insert(Computed(runtime.run(code).unwrap()));
        }
    }
}
