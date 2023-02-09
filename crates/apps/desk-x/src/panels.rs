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
use egui::{
    epaint::{PathShape, Shadow},
    style::Margin,
    Color32, Frame, Id, LayerId, Rect, Rounding, Sense, Stroke,
};
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
        let frame = Frame::side_top_panel(&backend.style()).stroke(Stroke::NONE);

        // Render the side panels
        let animated_width = egui::SidePanel::left("side_panel")
            .frame(frame)
            .exact_width(width)
            .resizable(false)
            .show_animated(&backend, expanded, |ui| {
                // fake the min rect to prevent the panel from expanding
                let mut ui = ui.child_ui(ui.max_rect(), Default::default());
                ui.set_max_width(MAX_WIDTH);
                if let Some(fps) = self.fps {
                    ui.label(format!("FPS: {:.1}", fps));
                }
            })
            .map(|res| res.response.rect.width())
            .unwrap_or_default();

        // Render the shadow
        let shadow = Shadow {
            extrusion: 10.0,
            color: Color32::from_black_alpha(255),
        };
        let rect = Rect::from_two_pos(
            (animated_width, 0.0).into(),
            (animated_width, backend.input().screen_rect().height()).into(),
        );
        let mesh = shadow.tessellate(rect, Rounding::none());
        backend.layer_painter(LayerId::background()).add(mesh);

        // Render the center panel
        let res = egui::CentralPanel::default()
            .show(&backend, |ui| {
                if ui.button("☰").clicked() {
                    if expanded {
                        width = 0.0;
                    } else {
                        width = DEFAULT_WIDTH;
                    }
                }
            })
            .response
            .interact(Sense::drag());
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
