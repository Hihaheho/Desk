mod compile;
mod editor_state;
mod editor_widget;
mod runtime;

use bevy::prelude::*;
use desk_plugin::DeskSystem;
use desk_window::window::Window;
use desk_window::{widget::WidgetId, window::DefaultWindow};
use dworkspace::Workspace;
use editor_state::EditorState;
use editor_widget::EditorWidget;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.before(DeskSystem::UpdateWidget))
            // move this to proper plugin
            .add_system(
                process_kernel
                    .label(DeskSystem::ProcessKernel)
                    .after(DeskSystem::RenderWidget),
            )
            .add_system(compile_system)
            .add_system(editor.label(DeskSystem::UpdateWidget));
    }
}

pub fn setup(mut kernel: Query<&mut Workspace, Added<Workspace>>) {
    for mut kernel in kernel.iter_mut() {
        kernel.add_state(EditorState::default());
    }
}

pub fn process_kernel(mut kernel: Query<&mut Workspace>) {
    for mut kernel in kernel.iter_mut() {
        kernel.process();
    }
}

pub fn compile_system(mut kernel: Query<&mut Workspace>) {
    for _kernel in kernel.iter_mut() {}
}

pub fn editor(mut window: Query<(&mut Window<egui::Context>, &Workspace), With<DefaultWindow>>) {
    if let Ok((mut window, kernel)) = window.get_single_mut() {
        for (id, _node) in kernel.snapshot.flat_nodes.iter() {
            window.add_widget(
                WidgetId::new(),
                EditorWidget {
                    node_id: id.clone(),
                },
            );
        }
    }
}
