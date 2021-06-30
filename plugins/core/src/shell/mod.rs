use bevy::prelude::*;
use language::code::node::Node;
use physics::widget::{component::Component, Widget};
use shell::{
    card::{render_card, Card, Computed},
    terminal::{render_terminal, Terminal},
};

pub struct ShellPlugin;

impl Plugin for ShellPlugin {
    fn build(&self, app: &mut bevy::app::AppBuilder) {
        app.add_startup_system(create_terminal.system())
            .add_system(terminal_rendering.system())
            .add_system(card_rendering.system());
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

fn create_terminal(mut commands: Commands) {
    commands.spawn_bundle(TerminalBundle::default());
}

fn terminal_rendering(mut commands: Commands, query: Query<(Entity, &Terminal, &Transform)>) {
    for (entity, terminal, transform) in query.iter() {
        if let Some(widget) = render_terminal(terminal, transform.translation.into()) {
            commands.entity(entity).insert(widget);
        }
    }
}

fn card_rendering(
    mut commands: Commands,
    query: Query<
        (Entity, &Card, &Node, Option<&Computed>, &Transform),
        Or<(
            Changed<Card>,
            Changed<Node>,
            Changed<Computed>,
            Changed<Transform>,
        )>,
    >,
) {
    for (entity, card, node, computed, transform) in query.iter() {
        if let Some(widget) = render_card(card, node, computed, transform.translation.into()) {
            commands.entity(entity).insert(widget);
        }
    }
}
