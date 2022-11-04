use bevy::prelude::*;

use desk_window::{
    ctx::Ctx,
    widget::{Widget, WidgetId},
    window::{DefaultWindow, Window},
};
use deskc_ids::{LinkName, NodeId};
use deskc_types::Type;
use dkernel::Kernel;
use dkernel_components::{content::Content, event::Event};
use system_ordering::DeskSystem;
pub struct TerminalPlugin;

impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(terminal.label(DeskSystem::UpdateWidget));
    }
}

fn terminal(mut window: Query<(&mut Window<egui::Context>, &Kernel), With<DefaultWindow>>) {
    if let Ok((mut window, _kernel)) = window.get_single_mut() {
        window.add_widget(WidgetId::new(), TerminalWidget);
    }
}

struct TerminalWidget;

impl Widget<egui::Context> for TerminalWidget {
    fn render(&mut self, ctx: &mut Ctx<egui::Context>) {
        egui::Window::new("Terminal").show(ctx.backend, |ui| {
            ui.label("Hello World");
            if ui.button("Add number").clicked() {
                ctx.add_event(Event::AddNode {
                    parent: None,
                    node_id: NodeId::new(),
                    content: Content::Integer(1),
                });
            }
            if ui.button("Add apply").clicked() {
                ctx.add_event(Event::AddNode {
                    parent: None,
                    node_id: NodeId::new(),
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
