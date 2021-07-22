#![allow(clippy::type_complexity)]
mod card;
mod event_handler;

use card::create_card;
pub use event_handler::*;

use plugin_core::{DeskSystem, ShellSystem};

use bevy::prelude::*;

use language::code::node::Code;
use physics::{
    shape::Shape,
    widget::{backend::Backends, component::Component, event::WidgetEvents, Widget, WidgetId},
    DragState, Velocity,
};
use runtime::card::{Card, Computed};
use shell_language::{render_node, CodeOperationHandler, CodeWidgetEventHandler};
use shell_terminal::render_terminal;
use terminal::terminal::Terminal;

pub struct ShellPlugin;

impl Plugin for ShellPlugin {
    fn build(&self, app: &mut bevy::app::AppBuilder) {
        app.init_resource::<Backends>()
            .add_startup_system(create_terminal.system())
            .add_startup_system(create_card.system())
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
            .add_system(
                widget_rendering
                    .system()
                    .label(DeskSystem::Shell)
                    .after(ShellSystem::UpdateWidget)
                    .label(ShellSystem::Render)
                    .before(ShellSystem::HandleEvents),
            )
            .add_system_set(
                SystemSet::new()
                    .label(DeskSystem::Shell)
                    .label(ShellSystem::HandleEvents)
                    .after(ShellSystem::Render)
                    .with_system(
                        EventHandlerPlugin::<CodeWidgetEventHandler>::default()
                            .system()
                            .label(ShellSystem::HandleEvents),
                    ),
            )
            .add_system_set(
                SystemSet::new()
                    .label(DeskSystem::HandleOperations)
                    .after(DeskSystem::Shell)
                    .before(DeskSystem::PrePhysics)
                    .with_system(
                        EventHandlerPlugin::<CodeOperationHandler>::default()
                            .system()
                            .label(ShellSystem::HandleEvents),
                    ),
            );
    }
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

#[derive(Bundle)]
struct WidgetBundle {
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

fn create_terminal(mut commands: Commands) {
    commands.spawn_bundle(TerminalBundle {
        transform: Transform::from_translation([100.0, 100.0, 0.].into()),
        ..Default::default()
    });
}

fn widget_adding_for_cards(mut command: Commands, query: Query<(Entity, &Card), Added<Card>>) {
    for (entity, card) in query.iter() {
        command
            .entity(entity)
            .insert(WidgetId::from(card.id.to_string()))
            .insert_bundle(WidgetBundle::default());
    }
}
fn widget_adding_for_terminal(mut command: Commands, query: Query<Entity, Added<Terminal>>) {
    for entity in query.iter() {
        command
            .entity(entity)
            .insert(WidgetId::from("terminal"))
            .insert_bundle(WidgetBundle::default());
    }
}

fn card_rendering(
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

fn terminal_rendering(mut query: Query<(&Terminal, &mut Component)>) {
    for (terminal, mut component) in query.iter_mut() {
        let new_component = render_terminal(terminal);
        if *component != new_component {
            *component = new_component;
        }
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
                *velocity = new_velocity;
            }
            if *drag_state != response.drag_state {
                *drag_state = response.drag_state.clone();
            }
            *widget_events = response.events;
        }
    }
}
