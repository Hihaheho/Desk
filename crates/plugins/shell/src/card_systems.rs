use bevy::prelude::*;
use language::{
    code::{
        node::{sugar as n, Code},
        operation::CodeOperations,
        CodeId,
    },
    Computed,
};
use physics::widget::{component::Component, WidgetId};
use shell_language::{render_node, CodeWidgetEventHandler};

use crate::widget_bundle::WidgetBundle;

#[derive(Bundle)]
struct CardBundle {
    code_id: CodeId,
    code: Code,
    transform: Transform,
    global_transform: GlobalTransform,
    code_operations: CodeOperations,
    widget_event_handler: CodeWidgetEventHandler,
}

impl Default for CardBundle {
    fn default() -> Self {
        CardBundle {
            code_id: CodeId::new(),
            code: n::string(""),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            code_operations: Default::default(),
            widget_event_handler: CodeWidgetEventHandler,
        }
    }
}

pub(crate) fn create_card(mut commands: Commands) {
    // commands.spawn_bundle(CardBundle {
    //     node: sugar::add(sugar::integer(1), sugar::integer(2)),
    //     transform: Transform::from_xyz(100.0, 300.0, 0.0),
    //     ..Default::default()
    // });

    commands.spawn_bundle(CardBundle {
        code: n::integer(1),
        transform: Transform::from_xyz(300.0, 200.0, 0.0),
        ..Default::default()
    });

    commands.spawn_bundle(CardBundle {
        code: n::string("aaaa"),
        transform: Transform::from_xyz(100.0, 500.0, 0.0),
        ..Default::default()
    });
}

pub(crate) fn widget_adding_for_cards(
    mut command: Commands,
    query: Query<(Entity, &CodeId), Added<CodeId>>,
) {
    for (entity, id) in query.iter() {
        command
            .entity(entity)
            .insert(WidgetId::from(id.to_string()))
            .insert_bundle(WidgetBundle::default());
    }
}

pub(crate) fn card_rendering(mut query: Query<(&Code, Option<&Computed>, &mut Component)>) {
    for (node, _computed, mut component) in query.iter_mut() {
        let new_component = render_node(node);
        if *component != new_component {
            *component = new_component;
        }
    }
}
