use core::DeskSystem;

use bevy::prelude::*;

use language::code::node::Node;
use physics::{
    shape::Shape,
    widget::{component::Component, Widget, WidgetId},
    Velocity,
};
use runtime::card::{Card, Computed};
use shell_language::render_node;
use shell_terminal::render_terminal;
use terminal::terminal::Terminal;

pub struct ShellPlugin;

#[derive(SystemLabel, PartialEq, Eq, Debug, Hash, Clone)]
enum ShellSystem {
    Add,
    Update,
}

impl Plugin for ShellPlugin {
    fn build(&self, app: &mut bevy::app::AppBuilder) {
        app.add_startup_system(create_terminal.system())
            .add_system_set(
                SystemSet::new()
                    .label(DeskSystem::Shell)
                    .label(ShellSystem::Add)
                    .with_system(terminal_rendering.system())
                    .with_system(card_rendering.system()),
            )
            .add_system_set(
                SystemSet::new()
                    .label(DeskSystem::Shell)
                    .after(ShellSystem::Add)
                    .label(ShellSystem::Update)
                    .with_system(widget_adding_for_cards.system())
                    .with_system(widget_adding_for_terminal.system()),
            );
    }
}

#[derive(Bundle)]
struct TerminalBundle {
    shell: Terminal,
    transform: Transform,
    global_transform: GlobalTransform,
    velocity: Velocity,
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
            velocity: Default::default(),
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
            .insert(Shape::default())
            .insert(WidgetId::from(card.id.to_string()))
            .insert(Component::default());
    }
}
fn widget_adding_for_terminal(mut command: Commands, query: Query<Entity, Added<Terminal>>) {
    for entity in query.iter() {
        command
            .entity(entity)
            .insert(Shape::default())
            .insert(WidgetId::from("terminal"))
            .insert(Component::default());
    }
}

fn card_rendering(
    mut query: Query<
        (&Node, Option<&Computed<Node>>, &mut Component),
        Or<(Changed<Node>, Changed<Computed<Node>>)>,
    >,
) {
    for (node, _computed, mut component) in query.iter_mut() {
        let new_component = render_node(node);
        // TODO: move this logic to apporopriate crate.
        if *component != new_component {
            *component = new_component;
        }
    }
}

fn terminal_rendering(mut query: Query<(&Terminal, &mut Component), Changed<Terminal>>) {
    for (terminal, mut component) in query.iter_mut() {
        let new_component = render_terminal(terminal);
        if *component != new_component {
            *component = new_component;
        }
    }
}
