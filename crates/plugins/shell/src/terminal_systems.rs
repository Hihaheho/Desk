use std::collections::HashMap;

use bevy::prelude::*;

use physics::{
    widget::{component::Component, WidgetId},
    Follow, FollowParams, Velocity,
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
                    velocity_offset: 80.0,
                    velocity_power: 1.6,
                    velocity_max: 1500.0,
                    velocity_coefficient: 0.4,
                    ..Default::default()
                },
            })
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

pub(crate) fn follow(
    mut query_set: QuerySet<(
        Query<(&mut Velocity, &Transform, &Follow<Entity>)>,
        Query<&Transform>,
    )>,
) {
    let mut entities = Vec::new();
    for (_, _, follow) in query_set.q0_mut().iter_mut() {
        entities.push(follow.target);
    }
    let mut vecs = HashMap::new();
    for entity in entities {
        if let Ok(transform) = query_set.q1().get(entity) {
            vecs.insert(entity, transform.translation.truncate());
        }
    }
    if let Ok((mut velocity, transform, follow)) = query_set.q0_mut().single_mut() {
        if let Some(target) = vecs.get(&follow.target) {
            let vec = transform.translation.truncate();
            *velocity = follow.parameters.follow_vector(&vec, target);
        }
    }
}
