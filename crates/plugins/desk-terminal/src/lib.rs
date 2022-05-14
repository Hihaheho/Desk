use bevy::prelude::*;

use command::terminal::Terminal;
use desk_window::{
    ctx::Ctx,
    widget::{Widget, WidgetId},
    window::{DefaultWindow, Window},
};
use dkernel::Kernel;
pub struct TerminalPlugin;

impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut App) {}
}

fn terminal(
    terminal: Query<&Terminal>,
    mut window: Query<(&mut Window<egui::Context>, &Kernel), With<DefaultWindow>>,
) {
    if let Ok(terminal) = terminal.get_single() {
        if let Ok((mut window, kernel)) = window.get_single_mut() {
            window.add_widget(WidgetId::new(), TerminalWidget());
        }
    }
}

struct TerminalWidget();

impl Widget<egui::Context> for TerminalWidget {
    fn render(&mut self, ctx: &Ctx<egui::Context>) {
        egui::Window::new("Terminal").show(ctx.backend, |ui| {
            ui.label("Hello World");
        });
    }
}
