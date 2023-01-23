use bevy::prelude::*;
use desk_plugin::DeskSystem;
use dworkspace::Workspace;

use desk_window::{
    ctx::Ctx,
    widget::{Widget, WidgetId},
    window::{DefaultWindow, Window},
};

#[derive(Component)]
pub struct About;

pub struct AboutPlugin;

impl Plugin for AboutPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(about.label(DeskSystem::UpdateWidget));
    }
}

fn about(mut window: Query<(&mut Window<egui::Context>, &Workspace), With<DefaultWindow>>) {
    if let Ok((mut window, _kernel)) = window.get_single_mut() {
        window.add_widget(WidgetId::new(), AboutWidget);
    }
}

struct AboutWidget;

impl Widget<egui::Context> for AboutWidget {
    fn render(&mut self, ctx: &mut Ctx<egui::Context>) {
        egui::Window::new("About").show(&ctx.backend(), |ui| {
            ui.label("Hello World");
        });
    }
}
