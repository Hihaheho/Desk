// This file is for prototyping.
// This will be moved to desk-editor later.

use bevy::prelude::*;

use desk_window::{
    ctx::Ctx,
    widget::{Widget, WidgetId},
    window::{DefaultWindow, Window},
};
use dworkspace::Workspace;

pub struct PlaygroundPlugin;

impl Plugin for PlaygroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut window: Query<(&mut Window<egui::Context>, &Workspace), With<DefaultWindow>>) {
    if let Ok((mut window, _kernel)) = window.get_single_mut() {
        window.add_widget(WidgetId::new(), PlaygroundWidget {});
    }
}

pub struct PlaygroundWidget {}

impl Widget<egui::Context> for PlaygroundWidget {
    fn render(&mut self, ctx: &mut Ctx<egui::Context>) {
        egui::Window::new("Editor").show(ctx.backend, |ui| {});
    }
}
