use bevy::prelude::*;

use physics::widget::{component::Component, WidgetId};
use shell_terminal::render_terminal;
use terminal::terminal::Terminal;

use crate::card_systems::WidgetBundle;

pub(crate) fn create_terminal(mut commands: Commands) {
    commands.spawn_bundle(TerminalBundle {
        transform: Transform::from_translation([100.0, 100.0, 0.].into()),
        ..Default::default()
    });
}

#[derive(Bundle)]
struct TerminalBundle {
    shell: Terminal,
    transform: Transform,
    global_transform: GlobalTransform,
}

impl Default for TerminalBundle {
    fn default() -> Self {
        TerminalBundle {
            shell: Terminal {
                // logs: vec![
                // prompt: Prompt::Default,
                // command_input: "".into(),
            },
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}
pub(crate) fn widget_adding_for_terminal(
    mut command: Commands,
    query: Query<Entity, Added<Terminal>>,
) {
    for entity in query.iter() {
        command
            .entity(entity)
            .insert(WidgetId::from("terminal"))
            .insert_bundle(WidgetBundle::default());
    }
}

pub(crate) fn terminal_rendering(mut query: Query<(&Terminal, &mut Component)>) {
    for (terminal, mut component) in query.iter_mut() {
        let new_component = render_terminal(terminal);
        if *component != new_component {
            *component = new_component;
        }
    }
}
