use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use desk_plugin::DeskSystem;
use desk_window::{
    ctx::Ctx,
    widget::{Widget, WidgetId},
    window::{DefaultWindow, Window},
};
use egui::{Id, Layout, Sense};
use once_cell::sync::Lazy;

pub struct PanelsPlugin;

impl Plugin for PanelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(system.label(DeskSystem::UpdateWidget));
    }
}

const SIDE_PANEL_WIDGET_ID: Lazy<WidgetId> = Lazy::new(|| WidgetId::new());

fn system(
    diagnostics: Res<Diagnostics>,
    mut window: Query<&mut Window<egui::Context>, With<DefaultWindow>>,
) {
    if let Ok(mut window) = window.get_single_mut() {
        window.add_widget(
            *SIDE_PANEL_WIDGET_ID,
            SystemWidget {
                fps: diagnostics
                    .get(FrameTimeDiagnosticsPlugin::FPS)
                    .and_then(|d| d.value()),
            },
        );
    }
}

struct SystemWidget {
    fps: Option<f64>,
}

const SIDE_PANEL_WIDTH: Lazy<Id> = Lazy::new(|| Id::new("side panel width"));
const MIN_WIDTH: f32 = 50.0;
const DEFAULT_WIDTH: f32 = 150.0;
const MAX_WIDTH: f32 = 300.0;

impl Widget<egui::Context> for SystemWidget {
    fn render(&mut self, ctx: &mut Ctx<egui::Context>) {
        let backend = ctx.backend();
        let mut width: f32 = backend
            .data()
            .get_persisted(*SIDE_PANEL_WIDTH)
            .unwrap_or_default();
        let expanded = width != 0.0;
        egui::SidePanel::left("side_panel")
            .exact_width(width)
            .resizable(false)
            .show_animated(&backend, expanded, |ui| {
                // fake the min rect to prevent the panel from expanding
                let mut ui = ui.child_ui(ui.max_rect(), Default::default());
                ui.set_max_width(MAX_WIDTH);
                if let Some(fps) = self.fps {
                    ui.label(format!("FPS: {:.1}", fps));
                }
            });

        let res = egui::CentralPanel::default()
            .show(&backend, |ui| {
                if ui.button("☰").clicked() {
                    if expanded {
                        width = 0.0;
                    } else {
                        width = DEFAULT_WIDTH;
                    }
                }
                ui.interact(ui.max_rect(), Id::new("central panel drag"), Sense::drag())
            })
            .inner;
        width = (width + res.drag_delta().x).max(0.0);
        if res.drag_released() {
            if width < MIN_WIDTH {
                width = 0.0;
            } else {
                width = width.max(DEFAULT_WIDTH);
            }
        }
        backend
            .data()
            .insert_persisted(*SIDE_PANEL_WIDTH, width.min(MAX_WIDTH));
    }
}
