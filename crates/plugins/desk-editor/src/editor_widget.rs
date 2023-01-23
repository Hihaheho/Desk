use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use bevy::prelude::Color;
use bevy::render::color;
use desk_theme::colorscheme::{CodeColorScheme, CodeColorTag as Tag};
use desk_theme::{EditorStyle, IndentGuide};
use desk_window::ctx::Ctx;
use desk_window::widget::Widget;
use deskc_ids::{LinkName, NodeId};
use dson::Dson;
use dworkspace::prelude::{EventPayload, FlatNode, UserId};
use dworkspace_codebase::code::SyntaxKind;
use dworkspace_codebase::content::Content;
use dworkspace_codebase::event::{Event, EventId};
use dworkspace_codebase::patch::{ContentPatch, OperandPatch};
use egui::epaint::TextShape;
use egui::{Color32, FontId, Layout, Rect, Rgba, RichText, Sense, Stroke, TextEdit, TextStyle};
use once_cell::sync::Lazy;

use crate::editor_state::{EditorState, NodeState};

pub struct EditorWidget {
    pub top_level_node_id: NodeId,
}

impl Widget<egui::Context> for EditorWidget {
    fn render(&mut self, ctx: &mut Ctx<egui::Context>) {
        egui::Window::new(self.top_level_node_id.0.to_string())
            .resizable(true)
            .vscroll(true)
            .show(&ctx.backend(), |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.set_max_width(1000.0);
                    let editor_style = EditorStyle::default();
                    let style = ui.style_mut();
                    style.spacing.item_spacing = egui::Vec2::new(0.0, editor_style.line_spacing);
                    style
                        .text_styles
                        .get_mut(&TextStyle::Monospace)
                        .unwrap()
                        .size = 14.0;
                    let mut colorscheme = CodeColorScheme::default();
                    colorscheme.set_comment(Color::hex("BA9AB9").unwrap());
                    colorscheme.set_control(Color::hex("E17092").unwrap());
                    colorscheme.set_declaration(Color::hex("E17092").unwrap());
                    colorscheme.set_default(Color::hex("333333").unwrap());
                    colorscheme.set_delimiter(Color::hex("777777").unwrap());
                    colorscheme.set_number(Color::hex("B08B35").unwrap());
                    colorscheme.set_operator(Color::hex("054C49").unwrap());
                    colorscheme.set_punct(Color::hex("777777").unwrap());
                    colorscheme.set_string(Color::hex("1F6E89").unwrap());
                    colorscheme.set(Tag::StringEscape, Color::hex("777777").unwrap());
                    colorscheme.set_symbol(Color::hex("777777").unwrap());
                    colorscheme.set_type(Color::hex("9466AA").unwrap());
                    colorscheme.set(Tag::Variable, Color::hex("112B4B").unwrap());
                    colorscheme.set(Tag::Label, Color::hex("74C9C1").unwrap());

                    let mut renderer = NodeRenderer {
                        ctx,
                        node_id: self.top_level_node_id,
                        ui,
                        indent: 0,
                        line_number: &mut 0,
                        chars: &mut 0,
                        editor_style: &editor_style,
                        needs_space: &mut false,
                        colorscheme: &colorscheme,
                        node_type: NodeType::Expr,
                    };
                    renderer.begin_line();
                    renderer.render();
                });
            });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum NodeType {
    Expr,
    Ty,
    Case,
    Handler,
}

struct Count {
    chars: u32,
    lines: u8,
}

struct NodeRenderer<'a, 'b> {
    ctx: &'a mut Ctx<'b, egui::Context>,
    node_id: NodeId,
    ui: &'a mut egui::Ui,
    indent: u8,
    line_number: &'a mut u8,
    chars: &'a mut u32,
    editor_style: &'a EditorStyle,
    needs_space: &'a mut bool,
    colorscheme: &'a CodeColorScheme,
    node_type: NodeType,
}

impl<'context> NodeRenderer<'_, 'context> {
    fn clone(&mut self) -> NodeRenderer<'_, 'context> {
        NodeRenderer {
            ctx: self.ctx,
            node_id: self.node_id,
            ui: self.ui,
            indent: self.indent,
            line_number: self.line_number,
            chars: self.chars,
            editor_style: self.editor_style,
            needs_space: self.needs_space,
            colorscheme: self.colorscheme,
            node_type: self.node_type,
        }
    }
    fn has_focus(&self) -> bool {
        self.state().selected_node == Some(self.node_id)
    }
    fn state(&self) -> &EditorState {
        self.ctx.workspace.get_state::<EditorState>().unwrap()
    }
    fn state_mut(&mut self) -> &mut EditorState {
        self.ctx.workspace.get_state_mut::<EditorState>().unwrap()
    }
    fn node_state_mut(&mut self) -> &mut NodeState {
        let node_id = self.node_id;
        self.state_mut().node_mut(node_id)
    }
    fn begin_line(&mut self) {
        *self.needs_space = false;
        let line_number = self.line_number.to_string();
        let shape = self.ui.monospace(&line_number);
        self.space(4 - line_number.len());
        for _ in 0..self.indent {
            let from = self.ui.cursor().min;
            let res = self
                .ui
                .monospace(" ".repeat(self.editor_style.indent_width as usize));
            if let IndentGuide::SingleColorLine { size } = self.editor_style.indent_guide {
                let to = from + (0.0, res.rect.height()).into();
                self.ui.painter().line_segment(
                    [from, to],
                    Stroke::new(size, self.ui.style().visuals.text_color()),
                );
            }
        }
    }
    fn new_line(&mut self) {
        *self.line_number += 1;
        let max = self.ui.max_rect();
        let avail = max.right_top().x - self.ui.cursor().min.x;
        self.ui.allocate_space((avail, 1.0).into());
        self.begin_line();
    }
    fn space(&mut self, n: usize) {
        self.ui.monospace(" ".repeat(n));
        *self.needs_space = false;
    }
    fn monotext(
        &mut self,
        text: impl Into<RichText>,
        tag: Tag,
        needs_space_before: bool,
        needs_space_after: bool,
    ) -> egui::Response {
        let mut text: RichText = text.into();
        *self.chars += text.text().len() as u32;
        let color = self.colorscheme.get(tag).as_linear_rgba_f32();
        text = text.color(Rgba::from_rgba_unmultiplied(
            color[0], color[1], color[2], color[3],
        ));

        if *self.needs_space && needs_space_before {
            self.ui.monospace(" ");
            *self.chars += 1;
        }
        let res = self.ui.monospace(text);
        *self.needs_space = needs_space_after;
        res
    }
    fn line_split(&mut self) -> bool {
        self.node_state_mut().line_split
    }
    fn set_line_split(&mut self, split: bool) {
        self.node_state_mut().line_split = split;
    }
    fn error(&mut self, message: &str) {
        let color = self.ui.style().visuals.error_fg_color;
        self.ui.colored_label(color, message);
    }
    fn render_operand(&mut self, node_id: NodeId) {
        let mut renderer = NodeRenderer {
            node_id,
            ..self.clone()
        };
        renderer.render()
    }
    fn with_new_indent(&mut self) -> NodeRenderer<'_, 'context> {
        let mut renderer = self.clone();
        renderer.indent += 1;
        renderer
    }
    fn with_node_type(&mut self, node_type: NodeType) -> NodeRenderer<'_, 'context> {
        let mut renderer = self.clone();
        renderer.node_type = node_type;
        renderer
    }
    fn count(&mut self, f: impl FnOnce(&mut Self)) -> Count {
        let start_chars = *self.chars;
        let start_lines = *self.line_number;
        f(self);
        Count {
            lines: *self.line_number - start_lines,
            chars: *self.chars - start_chars,
        }
    }
    fn render(&mut self) {
        let node = self
            .ctx
            .workspace
            .snapshot
            .flat_nodes
            .get(&self.node_id)
            .unwrap()
            .clone();
        match &node.content {
            Content::SourceCode {
                source: original,
                syntax,
            } => self.render_source_code(&node, original, syntax),
            Content::String(original) => self.render_string(&node, original),
            Content::Integer(original) => self.render_integer(&node, original),
            Content::Rational(a, b) => self.render_rational(&node, *a, *b),
            Content::Real(real) => self.render_real(&node, *real),
            Content::Apply { link_name } => self.render_apply(&node, link_name),
            Content::Do => self.render_do(&node),
            Content::Let => self.render_let(&node),
            Content::Perform => self.render_perform(&node),
            Content::Continue => self.render_continue(&node),
            Content::Handle => self.render_handle(&node),
            Content::Product => self.render_product(&node),
            Content::Match => self.render_match(&node),
            Content::Typed => self.render_typed(&node),
            Content::Hole => self.render_hole(&node),
            Content::Function => self.render_function(&node),
            Content::Vector => self.render_vector(&node),
            Content::Map => self.render_map(&node),
            Content::MapElem => self.render_map_elem(&node),
            Content::Case => self.render_case(&node),
            Content::Handler => self.render_handler(&node),
            Content::Effect => self.render_effect(&node),
            Content::DeclareBrand { brand } => self.render_declare_brand(&node, brand),
            Content::Label { label } => self.render_label(&node, label),
            Content::NewType { ident } => self.render_new_type(&node, ident),
            Content::TyLabeled { brand } => self.render_ty_labeled(&node, brand),
            Content::TyMap => self.render_ty_map(&node),
            Content::TyVector => self.render_ty_vector(&node),
            Content::TyProduct => self.render_ty_product(&node),
            Content::Sum => self.render_sum(&node),
            Content::TyLet { ident } => self.render_ty_let(&node, ident),
            Content::TyReal => self.render_ty_real(&node),
            Content::TyRational => self.render_ty_rational(&node),
            Content::TyInteger => self.render_ty_integer(&node),
            Content::TyString => self.render_ty_string(&node),
            Content::TyEffectful => self.render_ty_effectful(&node),
            Content::Effects => self.render_effects(&node),
            Content::EAdd => self.render_eadd(&node),
            Content::ESub => self.render_esub(&node),
            Content::EApply => self.render_eapply(&node),
            Content::Infer => self.render_infer(&node),
            Content::TyFunction => self.render_ty_function(&node),
            Content::Variable { ident } => self.render_variable(&node, ident),
        }
    }
    fn handle_remaining_operands<'a>(&mut self, mut remainder: impl Iterator<Item = &'a NodeId>) {
        let Some(&operand) = remainder.next() else {
            return;
        };
        self.error("unhandled children:");
        self.render_operand(operand);
        for child in remainder {
            self.render_operand(*child);
        }
    }

    fn render_source_code(&mut self, node: &FlatNode, original: &String, syntax: &SyntaxKind) {
        let mut source = original.clone();
        self.ui.code_editor(&mut source);
        if *original != source {
            self.ctx.add_event(Event {
                id: EventId::new(),
                user_id: self.ctx.user_id,
                payload: EventPayload::PatchContent {
                    node_id: self.node_id.clone(),
                    patch: ContentPatch::Replace(Content::SourceCode {
                        source,
                        syntax: syntax.clone(),
                    }),
                },
            });
        }
        self.handle_remaining_operands(node.operands.iter())
    }

    fn render_string(&mut self, node: &FlatNode, original: &String) {
        let mut string = original.clone();
        self.ui.text_edit_singleline(&mut string);
        if *original != string {
            self.ctx.add_event(Event {
                id: EventId::new(),
                user_id: self.ctx.user_id,
                payload: EventPayload::PatchContent {
                    node_id: self.node_id.clone(),
                    patch: ContentPatch::Replace(Content::String(string)),
                },
            });
        }
        self.handle_remaining_operands(node.operands.iter())
    }

    fn render_integer(&mut self, node: &FlatNode, original: &i64) {
        self.render_input(Tag::IntegerLiteral, || original.to_string());
        self.handle_remaining_operands(node.operands.iter())
    }

    fn render_rational(&mut self, node: &FlatNode, a: i64, b: u64) {
        self.render_input(Tag::RationalLiteral, || format!("{}/{}", a, b));
        self.handle_remaining_operands(node.operands.iter())
    }

    fn render_real(&mut self, node: &FlatNode, original: f64) {
        self.render_input(Tag::RealLiteral, || original.to_string());
        self.handle_remaining_operands(node.operands.iter())
    }
    fn render_input(&mut self, tag: Tag, default_text: impl FnOnce() -> String) {
        let text = self.node_state_mut().editing_text.clone();
        if let Some(mut text) = text {
            if *self.needs_space {
                self.space(1);
            }
            *self.needs_space = true;

            let res = TextEdit::singleline(&mut text)
                .font(TextStyle::Monospace)
                .desired_width(0.0)
                .cursor_at_end(true)
                .show(self.ui.deref_mut());
            if res.response.lost_focus() {
                self.node_state_mut().editing_text = None;
                // TODO
            } else {
                self.node_state_mut().editing_text = Some(text);
            }
        } else {
            let text = default_text();
            if self.monotext(&text, tag, true, true).clicked() {
                self.node_state_mut().editing_text = Some(text);
            }
        }
    }
    fn render_apply(&mut self, node: &FlatNode, link_name: &LinkName) {
        let reference = node.operands.len() == 1;
        let mut operands = node.operands.iter();
        let Some(ty) = operands.next() else {
            self.error("missing type");
            self.handle_remaining_operands(operands);
            return;
        };
        if reference {
            self.monotext("&", Tag::Reference, true, false);
        }
        self.render_operand(*ty);
        if !reference {
            self.monotext("(", Tag::ParamsDelimiter, false, false);
        }
        for operand in operands {
            self.render_operand(*operand);
        }
        if !reference {
            self.monotext(")", Tag::ParamsDelimiter, false, true);
        }
    }
    fn render_do(&mut self, node: &FlatNode) {
        todo!()
    }
    fn render_let(&mut self, node: &FlatNode) {
        self.monotext("let", Tag::Let, true, true);
        let mut operands = node.operands.iter();
        let Some(def) = operands.next() else {
            self.error("missing def");
            return;
        };
        self.render_operand(*def);
        self.monotext(";", Tag::StmtEnd, false, true);
        self.new_line();
        let Some(&expr) = operands.next() else {
            self.error("missing expr");
            return;
        };
        self.render_operand(expr);
    }
    fn render_perform(&mut self, node: &FlatNode) {
        todo!()
    }
    fn render_continue(&mut self, node: &FlatNode) {
        todo!()
    }
    fn render_handle(&mut self, node: &FlatNode) {
        todo!()
    }
    fn render_product(&mut self, node: &FlatNode) {
        let split = self.line_split();
        let items = node.operands.len();
        let mut max_length = 0;
        let mut operands = node.operands.iter();

        let Some(&operand) = operands.next() else {
            self.monotext("⊤", Tag::Product, true, true);
            return;
        };

        let mut renderer = if split {
            self.with_new_indent()
        } else {
            self.clone()
        };

        if split {
            renderer.new_line();
        }
        renderer.render_operand(operand);

        for operand in operands {
            renderer.monotext("*", Tag::Product, true, true);
            if split {
                renderer.new_line();
            }
            let count = renderer.count(|ctx| ctx.render_operand(*operand));
            max_length = max_length.max(count.chars);
        }
        if split {
            self.new_line();
        }
        if items >= 2 && max_length > 16 {
            self.set_line_split(true);
        }
    }
    fn render_match(&mut self, node: &FlatNode) {
        let mut operands = node.operands.iter();

        self.monotext("match", Tag::Match, true, true);

        let Some(&expr) = operands.next() else {
            self.error("missing expr");
            return;
        };

        self.render_operand(expr);

        self.monotext("{", Tag::ControlDelimiter, true, false);

        let mut renderer = self.with_new_indent();
        let mut renderer = renderer.with_node_type(NodeType::Case);

        for operand in operands {
            renderer.new_line();
            renderer.render_operand(*operand);
        }

        self.new_line();
        self.monotext("}", Tag::ControlDelimiter, true, false);
    }
    fn render_typed(&mut self, node: &FlatNode) {
        let mut operands = node.operands.iter();

        self.monotext("<", Tag::TypeDelimiter, true, false);
        let Some(&ty) = operands.next() else {
            self.error("missing ty");
            return;
        };
        self.render_operand(ty);
        self.monotext(">", Tag::TypeDelimiter, false, true);
        let Some(&expr) = operands.next() else {
            self.error("missing expr");
            return;
        };
        self.render_operand(expr);
        self.handle_remaining_operands(operands);
    }
    fn render_hole(&mut self, node: &FlatNode) {
        self.monotext("?", Tag::Hole, true, true);
        self.handle_remaining_operands(node.operands.iter());
    }
    fn render_function(&mut self, node: &FlatNode) {
        let mut operands = node.operands.iter();

        self.monotext("λ", Tag::Function, true, true);

        let Some(&param) = operands.next() else {
            self.error("missing param");
            return;
        };
        self.render_operand(param);

        self.monotext("→", Tag::FArrow, true, true);

        let Some(&body) = operands.next() else {
            self.error("missing body");
            return;
        };
        self.render_operand(body);
    }
    fn render_vector(&mut self, node: &FlatNode) {
        todo!()
    }
    fn render_map(&mut self, node: &FlatNode) {
        todo!()
    }
    fn render_map_elem(&mut self, node: &FlatNode) {
        todo!()
    }
    fn render_case(&mut self, node: &FlatNode) {
        let mut operands = node.operands.iter();

        let Some(&ty) = operands.next() else {
            self.error("missing ty");
            return;
        };
        self.with_node_type(NodeType::Ty).render_operand(ty);
        self.monotext("⇒", Tag::Arrow, true, true);
        let Some(&expr) = operands.next() else {
            self.error("missing expr");
            return;
        };
        self.render_operand(expr);
        self.handle_remaining_operands(operands);
    }
    fn render_handler(&mut self, node: &FlatNode) {
        todo!()
    }
    fn render_effect(&mut self, node: &FlatNode) {
        todo!()
    }
    fn render_declare_brand(&mut self, node: &FlatNode, brand: &str) {
        todo!()
    }
    fn render_label(&mut self, node: &FlatNode, label: &str) {
        self.monotext("@", Tag::Label, true, false);
        self.monotext(label, Tag::Label, false, true);
        let mut operands = node.operands.iter();
        let Some(&expr) = operands.next() else {
            self.error("missing expr");
            return;
        };
        self.render_operand(expr);
        self.handle_remaining_operands(operands);
    }
    fn render_new_type(&mut self, node: &FlatNode, ident: &String) {
        let mut operands = node.operands.iter();

        self.monotext("type", Tag::NewType, true, true);
        self.monotext(ident, Tag::Variable, true, true);
        self.monotext("=", Tag::TyAnno, true, true);
        let Some(&ty) = operands.next() else {
            self.error("missing ty");
            return;
        };
        self.with_node_type(NodeType::Ty).render_operand(ty);
        self.monotext(";", Tag::StmtEnd, false, true);
        self.new_line();
        let Some(&expr) = operands.next() else {
            self.error("missing expr");
            return;
        };
        self.render_operand(expr);
        self.handle_remaining_operands(operands);
    }
    fn render_ty_labeled(&mut self, node: &FlatNode, brand: &str) {
        self.render_label(node, brand);
    }
    fn render_ty_map(&mut self, node: &FlatNode) {
        todo!()
    }
    fn render_ty_vector(&mut self, node: &FlatNode) {
        todo!()
    }
    fn render_ty_product(&mut self, node: &FlatNode) {
        self.render_product(node);
    }
    fn render_sum(&mut self, node: &FlatNode) {
        let mut operands = node.operands.iter();

        let Some(&operand) = operands.next() else {
            self.monotext("⊥", Tag::Sum, true, true);

            return;
        };
        self.render_operand(operand);

        for operand in operands {
            self.monotext("|", Tag::Sum, true, true);
            self.render_operand(*operand);
        }
    }
    fn render_ty_let(&mut self, node: &FlatNode, ident: &String) {
        todo!()
    }
    fn render_ty_real(&mut self, node: &FlatNode) {
        self.monotext("real", Tag::TyReal, true, true);
        self.handle_remaining_operands(node.operands.iter());
    }
    fn render_ty_rational(&mut self, node: &FlatNode) {
        self.monotext("rat", Tag::TyRational, true, true);
        self.handle_remaining_operands(node.operands.iter());
    }
    fn render_ty_integer(&mut self, node: &FlatNode) {
        self.monotext("int", Tag::TyInteger, true, true);
        self.handle_remaining_operands(node.operands.iter());
    }
    fn render_ty_string(&mut self, node: &FlatNode) {
        self.monotext("str", Tag::TyString, true, true);
        self.handle_remaining_operands(node.operands.iter());
    }
    fn render_ty_effectful(&mut self, node: &FlatNode) {
        todo!()
    }
    fn render_effects(&mut self, node: &FlatNode) {
        todo!()
    }
    fn render_eadd(&mut self, node: &FlatNode) {
        todo!()
    }
    fn render_esub(&mut self, node: &FlatNode) {
        todo!()
    }
    fn render_eapply(&mut self, node: &FlatNode) {
        todo!()
    }
    fn render_infer(&mut self, node: &FlatNode) {
        self.monotext("_", Tag::Infer, true, true);
        self.handle_remaining_operands(node.operands.iter());
    }
    fn render_ty_function(&mut self, node: &FlatNode) {
        self.render_function(node);
    }
    fn render_variable(&mut self, node: &FlatNode, ident: &String) {
        self.monotext(ident, Tag::Variable, true, true);
        self.handle_remaining_operands(node.operands.iter());
    }
    fn render_dson(&mut self, dson: &Dson) {
        self.monotext(&dson.to_string(), Tag::DsonTyInteger, true, true);
    }
}
