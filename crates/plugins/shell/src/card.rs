use bevy::prelude::*;
use language::code::node::{sugar as n, Code};
use runtime::card::Card;
use shell_language::{CodeOperationHandler, CodeWidgetEventHandler};

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
