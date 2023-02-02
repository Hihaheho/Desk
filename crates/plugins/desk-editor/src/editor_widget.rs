use std::borrow::BorrowMut;

use bevy::prelude::Color;
use desk_theme::colorscheme::{CodeColorScheme, CodeColorTag as Tag};
use desk_theme::{EditorStyle, IndentGuide};
use desk_window::ctx::Ctx;
use desk_window::widget::Widget;
use deskc_ids::{LinkName, NodeId};
use dson::Dson;
use dworkspace::prelude::{EventPayload, UserId};
use dworkspace_codebase::code::SyntaxKind;
use dworkspace_codebase::content::Content;
use dworkspace_codebase::event::{Event, EventId};
use dworkspace_codebase::node::Node;
use dworkspace_codebase::patch::ContentPatch;
use egui::epaint::RectShape;
use egui::{Key, Modifiers, Rgba, RichText, Stroke, TextEdit, TextStyle};
use maybe_owned::MaybeOwnedMut;

use crate::editor_state::{EditorState, NextPos, NodeState, WordCursor, WordSearch};

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

                    let node = ctx.workspace.node(self.top_level_node_id);
                    let user_id = ctx.workspace.user_id();
                    let mut editor_state =
                        &mut ctx.workspace.get_state_mut::<EditorState>().unwrap();
                    let mut word_cursor_distance = None;
                    let has_next_pos = editor_state.next_pos.is_some();
                    let mut renderer = NodeRenderer {
                        events: &mut ctx.events,
                        user_id,
                        state: &mut editor_state,
                        node: &node,
                        ui,
                        indent: 0,
                        line_number: &mut 0,
                        chars: &mut 0,
                        editor_style: &editor_style,
                        needs_space: &mut false,
                        colorscheme: &colorscheme,
                        node_type: NodeType::Expr,
                        parent_hovered: false,
                        node_render_state: NodeRenderState::default().into(),
                        word_cursor_distance: &mut word_cursor_distance,
                        forward_word_found: &mut false,
                    };
                    renderer.begin_line();
                    renderer.render();
                    if has_next_pos {
                        editor_state.next_pos = None;
                    }
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

#[derive(Default)]
struct NodeRenderState {
    /// The number of printed words for the current node.
    node_word_offset: u16,
}

struct NodeRenderer<'a> {
    events: &'a mut Vec<Event>,
    user_id: UserId,
    state: &'a mut EditorState,
    node: &'a Node,
    ui: &'a mut egui::Ui,
    indent: u8,
    line_number: &'a mut u8,
    chars: &'a mut u32,
    editor_style: &'a EditorStyle,
    needs_space: &'a mut bool,
    colorscheme: &'a CodeColorScheme,
    node_type: NodeType,
    parent_hovered: bool,
    node_render_state: MaybeOwnedMut<'a, NodeRenderState>,
    // Vec2 is the distance between the next cursor and the word.
    word_cursor_distance: &'a mut Option<f32>,
    forward_word_found: &'a mut bool,
}

impl<'context> NodeRenderer<'_> {
    fn clone(&mut self) -> NodeRenderer<'_> {
        NodeRenderer {
            events: self.events,
            user_id: self.user_id,
            state: self.state,
            node: self.node,
            ui: self.ui,
            indent: self.indent,
            line_number: self.line_number,
            chars: self.chars,
            editor_style: self.editor_style,
            needs_space: self.needs_space,
            colorscheme: self.colorscheme,
            node_type: self.node_type,
            parent_hovered: self.parent_hovered,
            node_render_state: MaybeOwnedMut::Borrowed(self.node_render_state.borrow_mut()),
            word_cursor_distance: self.word_cursor_distance,
            forward_word_found: self.forward_word_found,
        }
    }
    fn add_event(&mut self, payload: EventPayload) {
        self.events.push(Event {
            id: EventId::new(),
            user_id: self.user_id,
            payload,
        });
    }
    fn selected(&self) -> bool {
        self.state.selected_nodes.contains(&self.node.id)
    }
    fn hovered(&self) -> bool {
        self.state.hovered_node == Some(self.node.id)
    }
    fn node_state(&mut self) -> &mut NodeState {
        let node_id = self.node.id;
        self.state.node_mut(node_id)
    }
    fn begin_line(&mut self) {
        *self.needs_space = false;
        let line_number = self.line_number.to_string();
        self.ui.monospace(&line_number);
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
        let color = self.colorscheme.get(tag);
        text = text.color(egui_color(color)).monospace();
        if self.selected() {
            text = text.background_color(egui_color(self.editor_style.hovered_background));
        } else if self.hovered() {
            text = text.background_color(egui_color(self.editor_style.hovered_background));
        } else if self.parent_hovered {
            text = text.background_color(egui_color(self.editor_style.hovered_child_background));
        }

        if *self.needs_space && needs_space_before {
            self.ui.monospace(" ");
            *self.chars += 1;
        }
        let res = self
            .ui
            .add(egui::Label::new(text).sense(egui::Sense::click_and_drag()));
        if res.hovered() {
            self.state.hovered_node = Some(self.node.id);
        }
        if res.clicked() {
            self.state.word_cursor = Some(WordCursor {
                offset: self.node_render_state.node_word_offset,
                node_id: self.node.id,
            });
        }
        if let Some(WordCursor { node_id, offset }) = self.state.word_cursor {
            if self.node.id == node_id && offset == self.node_render_state.node_word_offset {
                let stroke = self.editor_style.cursor_word_outline;
                self.ui.painter().add(RectShape::stroke(
                    res.rect,
                    0.1,
                    Stroke::new(stroke.size, egui_color(stroke.color)),
                ));
                let mut input = self.ui.input_mut();
                let height = res.rect.height();
                if input.consume_key(Modifiers::NONE, Key::ArrowLeft) {
                    self.state.next_pos = Some(NextPos {
                        pos: res.rect.left_center() + (-1.0, 0.0).into(),
                        search: WordSearch::Backward,
                    });
                } else if input.consume_key(Modifiers::NONE, Key::ArrowRight) {
                    self.state.next_pos = Some(NextPos {
                        pos: res.rect.center(),
                        search: WordSearch::Forward,
                    });
                } else if input.consume_key(Modifiers::NONE, Key::ArrowUp) {
                    self.state.next_pos = Some(NextPos {
                        pos: res.rect.center() + (0.0, -height).into(),
                        search: WordSearch::Nearest,
                    });
                } else if input.consume_key(Modifiers::NONE, Key::ArrowDown) {
                    self.state.next_pos = Some(NextPos {
                        pos: res.rect.center() + (0.0, height).into(),
                        search: WordSearch::Nearest,
                    });
                }
            }
        }

        if let Some(next) = self.state.next_pos {
            match next {
                NextPos {
                    pos,
                    search: WordSearch::Backward,
                } => {
                    // Update the cursor until it finds a next word.
                    let left_top = res.rect.left_top();
                    if if res.rect.y_range().contains(&pos.y) {
                        left_top.x < pos.x
                    } else {
                        left_top.y < pos.y
                    } {
                        self.state.word_cursor = Some(WordCursor {
                            offset: self.node_render_state.node_word_offset,
                            node_id: self.node.id,
                        });
                    }
                }
                NextPos {
                    pos,
                    search: WordSearch::Forward,
                } => {
                    if !*self.forward_word_found {
                        let center = res.rect.center();
                        if if res.rect.y_range().contains(&pos.y) {
                            center.x > pos.x
                        } else {
                            center.y > pos.y
                        } {
                            self.state.word_cursor = Some(WordCursor {
                                offset: self.node_render_state.node_word_offset,
                                node_id: self.node.id,
                            });
                            *self.forward_word_found = true;
                        }
                    }
                }
                NextPos {
                    pos,
                    search: WordSearch::Nearest,
                } => {
                    if res.rect.y_range().contains(&pos.y) {
                        let center = res.rect.center();
                        let distance = (center - pos).length_sq();
                        match self.word_cursor_distance {
                            Some(d) if *d < distance => {}
                            _ => {
                                self.state.word_cursor = Some(WordCursor {
                                    offset: self.node_render_state.node_word_offset,
                                    node_id: self.node.id,
                                });
                                *self.word_cursor_distance = Some(distance);
                            }
                        }
                    }
                }
            }
        }
        *self.needs_space = needs_space_after;
        self.node_render_state.node_word_offset += 1;
        res
    }
    fn line_split(&mut self) -> bool {
        self.node_state().line_split
    }
    fn set_line_split(&mut self, split: bool) {
        self.node_state().line_split = split;
    }
    fn error(&mut self, message: &str) {
        let color = self.ui.style().visuals.error_fg_color;
        self.ui.colored_label(color, message);
    }
    fn render_operand(&mut self, node: &Node) {
        let mut renderer = NodeRenderer {
            node,
            parent_hovered: self.parent_hovered || self.state.hovered_node == Some(self.node.id),
            node_render_state: NodeRenderState::default().into(),
            ..self.clone()
        };
        renderer.render()
    }
    fn with_new_indent(&mut self) -> NodeRenderer<'_> {
        let mut renderer = self.clone();
        renderer.indent += 1;
        renderer
    }
    fn with_node_type(&mut self, node_type: NodeType) -> NodeRenderer<'_> {
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
        match &self.node.content {
            Content::SourceCode {
                source: original,
                syntax,
            } => self.render_source_code(original, syntax),
            Content::String(original) => self.render_string(original),
            Content::Integer(original) => self.render_integer(original),
            Content::Rational(a, b) => self.render_rational(*a, *b),
            Content::Real(real) => self.render_real(*real),
            Content::Apply { link_name } => self.render_apply(link_name),
            Content::Do => self.render_do(),
            Content::Let => self.render_let(),
            Content::Perform => self.render_perform(),
            Content::Continue => self.render_continue(),
            Content::Handle => self.render_handle(),
            Content::Product => self.render_product(),
            Content::Match => self.render_match(),
            Content::Typed => self.render_typed(),
            Content::Hole => self.render_hole(),
            Content::Function => self.render_function(),
            Content::Vector => self.render_vector(),
            Content::Map => self.render_map(),
            Content::MapElem => self.render_map_elem(),
            Content::Case => self.render_case(),
            Content::Handler => self.render_handler(),
            Content::Effect => self.render_effect(),
            Content::DeclareBrand { brand } => self.render_declare_brand(brand),
            Content::Label { label } => self.render_label(label),
            Content::NewType { ident } => self.render_new_type(ident),
            Content::TyLabeled { brand } => self.render_ty_labeled(brand),
            Content::TyMap => self.render_ty_map(),
            Content::TyVector => self.render_ty_vector(),
            Content::TyProduct => self.render_ty_product(),
            Content::Sum => self.render_sum(),
            Content::TyLet { ident } => self.render_ty_let(ident),
            Content::TyReal => self.render_ty_real(),
            Content::TyRational => self.render_ty_rational(),
            Content::TyInteger => self.render_ty_integer(),
            Content::TyString => self.render_ty_string(),
            Content::TyEffectful => self.render_ty_effectful(),
            Content::Effects => self.render_effects(),
            Content::EAdd => self.render_eadd(),
            Content::ESub => self.render_esub(),
            Content::EApply => self.render_eapply(),
            Content::Infer => self.render_infer(),
            Content::TyFunction => self.render_ty_function(),
            Content::Variable { ident } => self.render_variable(ident),
        }
    }
    fn handle_remaining_operands<'a>(&mut self, mut remainder: impl Iterator<Item = &'a Node>) {
        let Some(operand) = remainder.next() else {
            return;
        };
        self.error("unhandled children:");
        self.render_operand(operand);
        for child in remainder {
            self.render_operand(child);
        }
    }

    fn render_source_code(&mut self, original: &String, syntax: &SyntaxKind) {
        let mut source = original.clone();
        self.ui.code_editor(&mut source);
        if *original != source {
            self.add_event(EventPayload::PatchContent {
                node_id: self.node.id.clone(),
                patch: ContentPatch::Replace(Content::SourceCode {
                    source,
                    syntax: syntax.clone(),
                }),
            });
        }
        self.handle_remaining_operands(self.node.operands.iter())
    }

    fn render_string(&mut self, original: &String) {
        let mut string = original.clone();
        self.ui.text_edit_singleline(&mut string);
        if *original != string {
            self.add_event(EventPayload::PatchContent {
                node_id: self.node.id.clone(),
                patch: ContentPatch::Replace(Content::String(string)),
            });
        }
        self.handle_remaining_operands(self.node.operands.iter())
    }

    fn render_integer(&mut self, original: &i64) {
        self.render_input(Tag::IntegerLiteral, || original.to_string());
        self.handle_remaining_operands(self.node.operands.iter())
    }

    fn render_rational(&mut self, a: i64, b: u64) {
        self.render_input(Tag::RationalLiteral, || format!("{}/{}", a, b));
        self.handle_remaining_operands(self.node.operands.iter())
    }

    fn render_real(&mut self, original: f64) {
        self.render_input(Tag::RealLiteral, || original.to_string());
        self.handle_remaining_operands(self.node.operands.iter())
    }
    fn render_input(&mut self, tag: Tag, default_text: impl FnOnce() -> String) {
        let text = self.node_state().editing_text.clone();
        if let Some(mut text) = text {
            if *self.needs_space {
                self.space(1);
            }
            *self.needs_space = true;

            let res = TextEdit::singleline(&mut text)
                .font(TextStyle::Monospace)
                .desired_width(0.0)
                .cursor_at_end(true)
                .show(&mut self.ui);
            if res.response.lost_focus() {
                self.node_state().editing_text = None;
                // TODO
            } else {
                self.node_state().editing_text = Some(text);
            }
        } else {
            let text = default_text();
            if self.monotext(&text, tag, true, true).clicked() {
                self.node_state().editing_text = Some(text);
            }
        }
    }
    fn render_apply(&mut self, link_name: &LinkName) {
        let reference = self.node.operands.len() == 1;
        let mut operands = self.node.operands.iter();
        let Some(ty) = operands.next() else {
            self.error("missing type");
            self.handle_remaining_operands(operands);
            return;
        };
        if reference {
            self.monotext("&", Tag::Reference, true, false);
        }
        self.render_operand(ty);
        if !reference {
            self.monotext("(", Tag::ParamsDelimiter, false, false);
        }
        for operand in operands {
            self.render_operand(operand);
        }
        if !reference {
            self.monotext(")", Tag::ParamsDelimiter, false, true);
        }
    }
    fn render_do(&mut self) {
        todo!()
    }
    fn render_let(&mut self) {
        self.monotext("let", Tag::Let, true, true);
        let mut operands = self.node.operands.iter();
        let Some(def) = operands.next() else {
            self.error("missing def");
            return;
        };
        self.render_operand(def);
        self.monotext(";", Tag::StmtEnd, false, true);
        self.new_line();
        let Some(expr) = operands.next() else {
            self.error("missing expr");
            return;
        };
        self.render_operand(expr);
    }
    fn render_perform(&mut self) {
        todo!()
    }
    fn render_continue(&mut self) {
        todo!()
    }
    fn render_handle(&mut self) {
        todo!()
    }
    fn render_product(&mut self) {
        let split = self.line_split();
        let items = self.node.operands.len();
        let mut max_length = 0;
        let mut operands = self.node.operands.iter();

        let Some(operand) = operands.next() else {
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
            let count = renderer.count(|ctx| ctx.render_operand(operand));
            max_length = max_length.max(count.chars);
        }
        if split {
            self.new_line();
        }
        if items >= 2 && max_length > 16 {
            self.set_line_split(true);
        }
    }
    fn render_match(&mut self) {
        let mut operands = self.node.operands.iter();

        self.monotext("match", Tag::Match, true, true);

        let Some(expr) = operands.next() else {
            self.error("missing expr");
            return;
        };

        self.render_operand(expr);

        self.monotext("{", Tag::ControlDelimiter, true, false);

        let mut renderer = self.with_new_indent();
        let mut renderer = renderer.with_node_type(NodeType::Case);

        for operand in operands {
            renderer.new_line();
            renderer.render_operand(operand);
        }

        self.new_line();
        self.monotext("}", Tag::ControlDelimiter, true, false);
    }
    fn render_typed(&mut self) {
        let mut operands = self.node.operands.iter();

        self.monotext("<", Tag::TypeDelimiter, true, false);
        let Some(ty) = operands.next() else {
            self.error("missing ty");
            return;
        };
        self.render_operand(ty);
        self.monotext(">", Tag::TypeDelimiter, false, true);
        let Some(expr) = operands.next() else {
            self.error("missing expr");
            return;
        };
        self.render_operand(expr);
        self.handle_remaining_operands(operands);
    }
    fn render_hole(&mut self) {
        self.monotext("?", Tag::Hole, true, true);
        self.handle_remaining_operands(self.node.operands.iter());
    }
    fn render_function(&mut self) {
        let mut operands = self.node.operands.iter();

        self.monotext("λ", Tag::Function, true, true);

        let Some(param) = operands.next() else {
            self.error("missing param");
            return;
        };
        self.render_operand(param);

        self.monotext("→", Tag::FArrow, true, true);

        let Some(body) = operands.next() else {
            self.error("missing body");
            return;
        };
        self.render_operand(body);
    }
    fn render_vector(&mut self) {
        todo!()
    }
    fn render_map(&mut self) {
        todo!()
    }
    fn render_map_elem(&mut self) {
        todo!()
    }
    fn render_case(&mut self) {
        let mut operands = self.node.operands.iter();

        let Some(ty) = operands.next() else {
            self.error("missing ty");
            return;
        };
        self.with_node_type(NodeType::Ty).render_operand(ty);
        self.monotext("⇒", Tag::Arrow, true, true);
        let Some(expr) = operands.next() else {
            self.error("missing expr");
            return;
        };
        self.render_operand(expr);
        self.handle_remaining_operands(operands);
    }
    fn render_handler(&mut self) {
        todo!()
    }
    fn render_effect(&mut self) {
        todo!()
    }
    fn render_declare_brand(&mut self, brand: &str) {
        todo!()
    }
    fn render_label(&mut self, label: &str) {
        self.monotext("@", Tag::Label, true, false);
        self.monotext(label, Tag::Label, false, true);
        let mut operands = self.node.operands.iter();
        let Some(expr) = operands.next() else {
            self.error("missing expr");
            return;
        };
        self.render_operand(expr);
        self.handle_remaining_operands(operands);
    }
    fn render_new_type(&mut self, ident: &String) {
        let mut operands = self.node.operands.iter();

        self.monotext("type", Tag::NewType, true, true);
        self.monotext(ident, Tag::Variable, true, true);
        self.monotext("=", Tag::TyAnno, true, true);
        let Some(ty) = operands.next() else {
            self.error("missing ty");
            return;
        };
        self.with_node_type(NodeType::Ty).render_operand(ty);
        self.monotext(";", Tag::StmtEnd, false, true);
        self.new_line();
        let Some(expr) = operands.next() else {
            self.error("missing expr");
            return;
        };
        self.render_operand(expr);
        self.handle_remaining_operands(operands);
    }
    fn render_ty_labeled(&mut self, brand: &str) {
        self.render_label(brand);
    }
    fn render_ty_map(&mut self) {
        todo!()
    }
    fn render_ty_vector(&mut self) {
        todo!()
    }
    fn render_ty_product(&mut self) {
        self.render_product();
    }
    fn render_sum(&mut self) {
        let mut operands = self.node.operands.iter();

        let Some(operand) = operands.next() else {
            self.monotext("⊥", Tag::Sum, true, true);

            return;
        };
        self.render_operand(operand);

        for operand in operands {
            self.monotext("|", Tag::Sum, true, true);
            self.render_operand(operand);
        }
    }
    fn render_ty_let(&mut self, ident: &String) {
        todo!()
    }
    fn render_ty_real(&mut self) {
        self.monotext("real", Tag::TyReal, true, true);
        self.handle_remaining_operands(self.node.operands.iter());
    }
    fn render_ty_rational(&mut self) {
        self.monotext("rat", Tag::TyRational, true, true);
        self.handle_remaining_operands(self.node.operands.iter());
    }
    fn render_ty_integer(&mut self) {
        self.monotext("int", Tag::TyInteger, true, true);
        self.handle_remaining_operands(self.node.operands.iter());
    }
    fn render_ty_string(&mut self) {
        self.monotext("str", Tag::TyString, true, true);
        self.handle_remaining_operands(self.node.operands.iter());
    }
    fn render_ty_effectful(&mut self) {
        todo!()
    }
    fn render_effects(&mut self) {
        todo!()
    }
    fn render_eadd(&mut self) {
        todo!()
    }
    fn render_esub(&mut self) {
        todo!()
    }
    fn render_eapply(&mut self) {
        todo!()
    }
    fn render_infer(&mut self) {
        self.monotext("_", Tag::Infer, true, true);
        self.handle_remaining_operands(self.node.operands.iter());
    }
    fn render_ty_function(&mut self) {
        self.render_function();
    }
    fn render_variable(&mut self, ident: &String) {
        self.monotext(ident, Tag::Variable, true, true);
        self.handle_remaining_operands(self.node.operands.iter());
    }
    fn render_dson(&mut self, dson: &Dson) {
        self.monotext(&dson.to_string(), Tag::DsonTyInteger, true, true);
    }
}

fn egui_color(color: Color) -> Rgba {
    let color = color.as_linear_rgba_f32();
    Rgba::from_rgba_unmultiplied(color[0], color[1], color[2], color[3])
}
