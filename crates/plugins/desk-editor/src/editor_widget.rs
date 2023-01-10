use desk_window::ctx::Ctx;
use desk_window::widget::Widget;
use deskc_ids::NodeId;
use dworkspace_codebase::content::Content;
use dworkspace_codebase::event::Event;
use dworkspace_codebase::patch::{ContentPatch, OperandPatch};

use crate::editor_state::EditorState;

pub struct EditorWidget {
    pub node_id: NodeId,
}

impl Widget<egui::Context> for EditorWidget {
    fn render(&mut self, ctx: &mut Ctx<egui::Context>) {
        egui::Area::new(self.node_id.0.to_string()).show(ctx.backend, |ui| {
            ui.label("====");
            if let Some(node) = ctx.kernel.snapshot.flat_nodes.get(&self.node_id) {
                match &node.content {
                    Content::SourceCode {
                        source: original,
                        syntax,
                    } => {
                        let mut source = original.clone();
                        ui.text_edit_multiline(&mut source);
                        if *original != source {
                            ctx.kernel.commit(Event::PatchContent {
                                node_id: self.node_id.clone(),
                                patch: ContentPatch::Replace(Content::SourceCode {
                                    source,
                                    syntax: syntax.clone(),
                                }),
                            });
                        }
                    }
                    Content::String(original) => {
                        let mut string = original.clone();
                        ui.text_edit_singleline(&mut string);
                        if *original != string {
                            ctx.kernel.commit(Event::PatchContent {
                                node_id: self.node_id.clone(),
                                patch: ContentPatch::Replace(Content::String(string)),
                            });
                        }
                    }
                    Content::Integer(original) => {
                        let mut number = *original;
                        ui.add(egui::DragValue::new(&mut number));
                        if *original != number {
                            ctx.kernel.commit(Event::PatchContent {
                                node_id: self.node_id.clone(),
                                patch: ContentPatch::Replace(Content::Integer(number)),
                            });
                        }
                    }
                    Content::Rational(_a, _b) => todo!(),
                    Content::Real(_float) => todo!(),
                    Content::Apply { link_name } => {}
                    _ => todo!(),
                }
            }
            if let Some(target) = ctx
                .kernel
                .get_state::<EditorState>()
                .unwrap()
                .child_addition_target
                .clone()
            {
                if target != self.node_id && ui.button("Add this as a child").clicked() {
                    ctx.kernel.commit(Event::PatchOperand {
                        node_id: target,
                        patch: OperandPatch::Insert {
                            index: 0,
                            node_id: self.node_id.clone(),
                        },
                    });
                    ctx.kernel
                        .get_state_mut::<EditorState>()
                        .unwrap()
                        .child_addition_target = None;
                }
            }
        });
    }
}
