use desk_window::ctx::Ctx;
use desk_window::widget::Widget;
use deskc_ids::NodeId;
use dkernel_components::content::Content;
use dkernel_components::event::Event;
use dkernel_components::patch::{ChildrenPatch, ContentPatch};

use crate::editor_state::EditorState;

pub struct EditorWidget {
    pub node_id: NodeId,
}

impl Widget<egui::Context> for EditorWidget {
    fn render(&mut self, ctx: &mut Ctx<egui::Context>) {
        egui::Area::new(&self.node_id).show(ctx.backend, |ui| {
            ui.label("====");
            if let Some(node) = ctx.kernel.snapshot.flat_nodes.get(&self.node_id) {
                match &node.content {
                    dkernel_components::content::Content::Source(original) => {
                        let mut source = original.clone();
                        ui.text_edit_multiline(&mut source);
                        if *original != source {
                            ctx.kernel.commit(Event::PatchContent {
                                node_id: self.node_id.clone(),
                                patch: ContentPatch::Replace(Content::Source(source)),
                            });
                        }
                    }
                    dkernel_components::content::Content::String(original) => {
                        let mut string = original.clone();
                        ui.text_edit_singleline(&mut string);
                        if *original != string {
                            ctx.kernel.commit(Event::PatchContent {
                                node_id: self.node_id.clone(),
                                patch: ContentPatch::Replace(Content::String(string)),
                            });
                        }
                    }
                    dkernel_components::content::Content::Integer(original) => {
                        let mut number = *original;
                        ui.add(egui::DragValue::new(&mut number));
                        if *original != number {
                            ctx.kernel.commit(Event::PatchContent {
                                node_id: self.node_id.clone(),
                                patch: ContentPatch::Replace(Content::Integer(number)),
                            });
                        }
                    }
                    dkernel_components::content::Content::Rational(a, b) => todo!(),
                    dkernel_components::content::Content::Float(float) => todo!(),
                    dkernel_components::content::Content::Apply { ty, .. } => {
                        let mut clicked = None;
                        ui.label(format!("{:?}", ty));
                        for (index, child) in node.children.iter().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(format!("{:?}", child));
                                if ui.button("x").clicked() {
                                    clicked = Some(Event::PatchChildren {
                                        node_id: self.node_id.clone(),
                                        patch: ChildrenPatch::Remove { index },
                                    });
                                }
                            });
                        }
                        if let Some(event) = clicked {
                            ctx.kernel.commit(event);
                        }
                        if ui.button("add a node as a child").clicked() {
                            ctx.kernel
                                .get_state_mut::<EditorState>()
                                .unwrap()
                                .child_addition_target = Some(self.node_id.clone());
                        }
                    }
                }
            }
            if let Some(target) = &ctx
                .kernel
                .get_state::<EditorState>()
                .unwrap()
                .child_addition_target
            {
                if *target != self.node_id {
                    if ui.button("Add this as a child").clicked() {
                        ctx.kernel.commit(Event::PatchChildren {
                            node_id: target.clone(),
                            patch: ChildrenPatch::Insert {
                                index: 0,
                                node: self.node_id.clone(),
                            },
                        });
                        ctx.kernel
                            .get_state_mut::<EditorState>()
                            .unwrap()
                            .child_addition_target = None;
                    }
                }
            }
        });
    }
}
