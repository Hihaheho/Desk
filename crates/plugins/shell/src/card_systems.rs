use bevy::prelude::*;
use language::code::node::{sugar as n, Code};
use physics::{
    shape::Shape,
    widget::{component::Component, event::WidgetEvents, WidgetId},
    DragState, Velocity,
};
use runtime::card::{Card, Computed};
use shell_language::{render_node, CodeOperationHandler, CodeWidgetEventHandler};

#[derive(Bundle)]
struct CardBundle {
    card: Card,
    code: Code,
    transform: Transform,
    global_transform: GlobalTransform,
    widget_event_handler: CodeWidgetEventHandler,
    code_operation_handler: CodeOperationHandler,
}

impl Default for CardBundle {
    fn default() -> Self {
        CardBundle {
            card: Card::new(),
            code: n::string(""),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            widget_event_handler: CodeWidgetEventHandler,
            code_operation_handler: CodeOperationHandler,
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

#[derive(Bundle)]
pub(crate) struct WidgetBundle {
    shape: Shape,
    component: Component,
    drag_state: DragState,
    velocity: Velocity,
    events: WidgetEvents,
}

impl Default for WidgetBundle {
    fn default() -> Self {
        Self {
            shape: Default::default(),
            component: Default::default(),
            drag_state: Default::default(),
            velocity: Default::default(),
            events: Default::default(),
        }
    }
}

pub(crate) fn widget_adding_for_cards(
    mut command: Commands,
    query: Query<(Entity, &Card), Added<Card>>,
) {
    for (entity, card) in query.iter() {
        command
            .entity(entity)
            .insert(WidgetId::from(card.id.to_string()))
            .insert_bundle(WidgetBundle::default());
    }
}

pub(crate) fn card_rendering(
    mut query: Query<
        (&Code, Option<&Computed<Code>>, &mut Component),
        Or<(Changed<Code>, Changed<Computed<Code>>)>,
    >,
) {
    for (node, _computed, mut component) in query.iter_mut() {
        let new_component = render_node(node);
        if *component != new_component {
            *component = new_component;
        }
    }
}
