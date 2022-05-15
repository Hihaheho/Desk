mod editor_state;
mod editor_widget;

use bevy::prelude::*;
use desk_window::window::Window;
use desk_window::{widget::WidgetId, window::DefaultWindow};
use dkernel::Kernel;
use editor_state::EditorState;
use editor_widget::EditorWidget;
use system_ordering::DeskSystem;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.before(DeskSystem::UpdateWidget))
            .add_system(
                process_kernel
                    .label(DeskSystem::ProcessKernel)
                    .after(DeskSystem::RenderWidget),
            )
            .add_system(editor.label(DeskSystem::UpdateWidget));
    }
}

pub fn setup(mut kernel: Query<&mut Kernel, Added<Kernel>>) {
    for mut kernel in kernel.iter_mut() {
        kernel.add_state(EditorState::default());
    }
}

pub fn process_kernel(mut kernel: Query<&mut Kernel>) {
    for mut kernel in kernel.iter_mut() {
        kernel.process();
    }
}

pub fn editor(mut window: Query<(&mut Window<egui::Context>, &Kernel), With<DefaultWindow>>) {
    if let Ok((mut window, kernel)) = window.get_single_mut() {
        for (id, node) in kernel.snapshot.flat_nodes.iter() {
            window.add_widget(
                WidgetId::new(),
                EditorWidget {
                    node_id: id.clone(),
                },
            );
        }
    }
}
