use bevy::prelude::*;

use desk_plugin::DeskSystem;
use desk_window::{
    ctx::Ctx,
    widget::{Widget, WidgetId},
    window::{DefaultWindow, Window},
};
use dworkspace::Workspace;
pub struct TerminalPlugin;

impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(terminal.label(DeskSystem::UpdateWidget));
    }
}

fn terminal(mut window: Query<(&mut Window<egui::Context>, &Workspace), With<DefaultWindow>>) {
    if let Ok((mut window, _kernel)) = window.get_single_mut() {
        window.add_widget(WidgetId::new(), TerminalWidget);
    }
}

struct TerminalWidget;

impl Widget<egui::Context> for TerminalWidget {
    fn render(&mut self, ctx: &mut Ctx<egui::Context>) {
        egui::Window::new("Terminal").show(&ctx.backend(), |ui| {
            ui.label("Hello World");
        });
    }
}
