#![allow(clippy::type_complexity)]
mod card_systems;
mod cursor_systems;
mod event_handler;
mod terminal_systems;
mod widget_bundle;

use card_systems::{card_rendering, create_card, widget_adding_for_cards};
use cursor_systems::{add_cursor, move_cursor};
pub use event_handler::*;

use plugin_core::{DeskSystem, ShellSystem};

use bevy::prelude::*;

use physics::{
    shape::Shape,
    widget::{backend::Backends, event::WidgetEvents, Widget},
    DragState, Velocity,
};
use shell_language::CodeWidgetEventHandler;
use shell_terminal::TerminalWidgetEventHandler;
use terminal_systems::{
    create_terminal, follow, terminal_rendering, widget_adding_for_terminal,
};

pub struct ShellPlugin;

impl Plugin for ShellPlugin {
    fn build(&self, app: &mut bevy::app::AppBuilder) {
        app.init_resource::<Backends>()
            .add_startup_system(create_terminal.system())
            .add_startup_system(create_card.system())
            .add_startup_system(add_cursor.system())
            .add_system(
                move_cursor
                    .system()
                    .label(DeskSystem::UpdateStatesToLatest)
                    .before(DeskSystem::Shell),
            )
            .add_system(
                follow
                    .system()
                    .after(DeskSystem::Shell)
                    .before(DeskSystem::PrePhysics),
            )
            .add_system(reset_velocity.system().after(DeskSystem::PrePhysics))
            .add_system_set(
                SystemSet::new()
                    .label(DeskSystem::Shell)
                    .label(ShellSystem::Add)
                    .with_system(widget_adding_for_cards.system())
                    .with_system(widget_adding_for_terminal.system()),
            )
            .add_system_set(
                SystemSet::new()
                    .label(DeskSystem::Shell)
                    .after(ShellSystem::Add)
                    .label(ShellSystem::UpdateComponent)
                    .before(ShellSystem::UpdateWidget)
                    .with_system(terminal_rendering.system())
                    .with_system(card_rendering.system()),
            )
            .add_system_set(
                SystemSet::new()
                    .label(DeskSystem::Shell)
                    .after(ShellSystem::UpdateWidget)
                    .label(ShellSystem::Render)
                    .before(ShellSystem::HandleEvents)
                    .with_system(widget_rendering.system()),
            )
            .add_system_set(
                SystemSet::new()
                    .label(DeskSystem::Shell)
                    .after(ShellSystem::Render)
                    .label(ShellSystem::HandleEvents)
                    .before(DeskSystem::HandleOperations)
                    .with_system(
                        EventHandlerWrapper::<CodeWidgetEventHandler>::default()
                            .event_handler_system(),
                    )
                    .with_system(
                        EventHandlerWrapper::<TerminalWidgetEventHandler>::default()
                            .event_handler_system(),
                    ),
            );
    }
}

fn reset_velocity(mut query: Query<&mut Velocity>) {
    for mut velocity in query.iter_mut() {
        velocity.0 = Vec2::ZERO;
    }
}

fn widget_rendering(
    time: Res<Time>,
    mut backends: ResMut<Backends>,
    mut query: Query<(
        &Widget,
        &mut Shape,
        &mut Velocity,
        &mut DragState,
        &mut WidgetEvents,
    )>,
) {
    for (widget, mut shape, mut velocity, mut drag_state, mut widget_events) in query.iter_mut() {
        if let Some(backend) = backends.get_mut(&widget.backend_id) {
            let response = backend.render(widget);
            if *shape != response.shape {
                *shape = response.shape.clone();
            }

            let new_velocity = (response.drag_delta / time.delta_seconds()).into();
            if *velocity != new_velocity {
                *velocity = &*velocity + new_velocity;
            }
            if *drag_state != response.drag_state {
                *drag_state = response.drag_state.clone();
            }
            *widget_events = response.events;
        }
    }
}
