use bevy::prelude::*;

use physics::{
    widget::{component::Component, WidgetId},
    DragState, Follow, FollowParams,
};
use shell_terminal::{render_terminal, TerminalWidgetEventHandler};
use terminal::{terminal::Terminal, Cursor, TerminalOperations};

use crate::widget_bundle::WidgetBundle;

pub(crate) fn create_terminal(mut commands: Commands) {
    commands.spawn_bundle(TerminalBundle {
        transform: Transform::from_translation([100.0, 100.0, 0.].into()),
        ..Default::default()
    });
}

#[derive(Bundle, Default)]
struct TerminalBundle {
    shell: Terminal,
    terminal_operations: TerminalOperations,
    transform: Transform,
    global_transform: GlobalTransform,
    widget_event_handler: TerminalWidgetEventHandler,
}
pub(crate) fn widget_adding_for_terminal(
    mut command: Commands,
    cursor: Query<Entity, With<Cursor>>,
    query: Query<Entity, Added<Terminal>>,
) {
    for entity in query.iter() {
        command
            .entity(entity)
            .insert(WidgetId::from("terminal"))
            .insert(Follow {
                target: cursor.single().unwrap(),
                parameters: FollowParams {
                    ignore_area_size: 80.0,
                    velocity_power: 1.6,
                    velocity_max: 1500.0,
                    velocity_coefficient: 0.4,
                    ..Default::default()
                },
            })
            .insert_bundle(WidgetBundle::default())
            .remove::<DragState>();
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
