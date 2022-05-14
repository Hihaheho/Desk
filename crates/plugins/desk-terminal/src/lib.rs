use bevy::prelude::*;


use desk_window::{
    ctx::Ctx,
    widget::{Widget, WidgetId},
    window::{DefaultFile, DefaultWindow, Window},
};
use deskc_ids::{FileId, LinkName, NodeId};
use deskc_types::Type;
use dkernel::Kernel;
use dkernel_components::{content::Content, event::Event};
pub struct TerminalPlugin;

impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(terminal);
    }
}

fn terminal(
    mut window: Query<(&mut Window<egui::Context>, &Kernel, &DefaultFile), With<DefaultWindow>>,
) {
    if let Ok((mut window, _kernel, default_file)) = window.get_single_mut() {
        window.add_widget(
            WidgetId::new(),
            TerminalWidget {
                default_file_id: default_file.0.clone(),
            },
        );
    }
}

struct TerminalWidget {
    default_file_id: FileId,
}

impl Widget<egui::Context> for TerminalWidget {
    fn render(&mut self, ctx: &mut Ctx<egui::Context>) {
        egui::Window::new("Terminal").show(ctx.backend, |ui| {
            ui.label("Hello World");
            if ui.button("Add number").clicked() {
                ctx.add_event(Event::AddNode {
                    node_id: NodeId::new(),
                    file_id: self.default_file_id.clone(),
                    content: Content::Integer(1),
                });
            }
            if ui.button("Add apply").clicked() {
                ctx.add_event(Event::AddNode {
                    node_id: NodeId::new(),
                    file_id: self.default_file_id.clone(),
                    content: Content::Apply {
                        ty: Type::Function {
                            parameters: vec![Type::Number, Type::Number],
                            body: Box::new(Type::Label {
                                label: "sum".into(),
                                item: Box::new(Type::Number),
                            }),
                        },
                        link_name: LinkName::None,
                    },
                });
            }
        });
    }
}
