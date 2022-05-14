use bevy::prelude::*;
use desk_window::ctx::Ctx;
use desk_window::window::Window;
use desk_window::{
    widget::{Widget, WidgetId},
    window::DefaultWindow,
};
use deskc_ids::NodeId;
use dkernel::Kernel;
use dkernel_components::flat_node::FlatNode;
use system_ordering::DeskSystem;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            process_kernel
                .label(DeskSystem::ProcessKernel)
                .after(DeskSystem::RenderWidget),
        )
        .add_system(editor.label(DeskSystem::UpdateWidget));
    }
}

pub fn process_kernel(mut kernel: Query<&mut Kernel>) {
    for mut kernel in kernel.iter_mut() {
        kernel.process();
    }
}

pub fn editor(mut window: Query<(&mut Window<egui::Context>, &Kernel), With<DefaultWindow>>) {
    if let Ok((mut window, kernel)) = window.get_single_mut() {
        window.add_widget(
            WidgetId::new(),
            EditorWidget {
                flat_nodes: kernel
                    .snapshot
                    .flat_nodes
                    .iter()
                    .map(|(id, node)| (id.clone(), node.clone()))
                    .collect(),
            },
        );
    }
}

pub struct EditorWidget {
    flat_nodes: Vec<(NodeId, FlatNode)>,
}

impl Widget<egui::Context> for EditorWidget {
    fn render(&mut self, ctx: &mut Ctx<egui::Context>) {
        for (id, node) in &self.flat_nodes {
            egui::Window::new(format!("{:?}", id)).show(ctx.backend, |ui| {
                ui.label(format!("{:?}", node));
            });
        }
    }
}
