use bevy::prelude::*;
use shell::terminal::Terminal;

struct ShellPlugin;

impl Plugin for ShellPlugin {
    fn build(&self, app: &mut bevy::app::AppBuilder) {
        app.add_startup_system(create_terminal.system())
            .add_system(render_terminal.system());
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

fn render_terminal(mut commands: Commands, query: Query<(Entity, &Terminal)>) {
    for (entity, shell) in query.iter() {
        // commands.entity(entity).insert(Widget {
        //     id: "desk shell".into(),
        //     position: Vec2::new(10.0, 10.0),
        //     shape: None,
        //     component: Component,
        // });
    }
}
