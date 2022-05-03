use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Color32},
    EguiContext,
};
#[cfg(feature = "inspector")]
use bevy_inspector_egui::WorldInspectorPlugin;
use theme::Theme;

pub struct EguiPlugin;

impl Plugin for EguiPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "inspector")]
        app.add_plugin(WorldInspectorPlugin::new());
        app.add_plugin(bevy_egui::EguiPlugin)
            .register_type::<Theme>()
            .add_startup_system(add_theme)
            .add_system(egui_theme)
            .add_system(ui_example);
    }
}

fn ui_example(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Hello")
        .current_pos((100.0, 100.0))
        .show(egui_context.ctx_mut(), |ui| {
            ui.label("world");
        });
}

fn add_theme(mut commands: Commands) {
    commands
        .spawn()
        .insert(ron::from_str::<Theme>(include_str!("../../../../../configs/theme.ron")).unwrap());
}

fn color(color: &Color) -> Color32 {
    Color32::from_rgba_unmultiplied(
        (color.r() * 256.0) as u8,
        (color.g() * 256.0) as u8,
        (color.b() * 256.0) as u8,
        (color.a() * 256.0) as u8,
    )
}

fn egui_theme(mut egui_context: ResMut<EguiContext>, theme: Query<&Theme, Changed<Theme>>) {
    if let Ok(theme) = theme.get_single() {
        #[cfg(not(feature = "web"))]
        {
            let theme_ron = ron::ser::to_string_pretty(&*theme, Default::default()).unwrap();
            std::fs::write("configs/theme.ron", theme_ron).unwrap();
        }

        let mut style = bevy_egui::egui::Style::default();
        style.visuals.window_shadow.color = color(&theme.window_shadow.color);
        style.visuals.window_shadow.extrusion = theme.window_shadow.extrusion;

        style.visuals.window_rounding.ne = theme.window_corner_radius;
        style.visuals.window_rounding.nw = theme.window_corner_radius;
        style.visuals.window_rounding.se = theme.window_corner_radius;
        style.visuals.window_rounding.sw = theme.window_corner_radius;

        style.visuals.widgets.active.bg_fill = color(&theme.active.background);
        style.visuals.widgets.active.bg_stroke.color = color(&theme.active.border.color);
        style.visuals.widgets.active.bg_stroke.width = theme.active.border.size;
        style.visuals.widgets.active.fg_stroke.color = color(&theme.active.stroke.color);
        style.visuals.widgets.active.fg_stroke.width = theme.active.stroke.size;

        style.visuals.widgets.inactive.bg_fill = color(&theme.inactive.background);
        style.visuals.widgets.inactive.bg_stroke.color = color(&theme.inactive.border.color);
        style.visuals.widgets.inactive.bg_stroke.width = theme.inactive.border.size;
        style.visuals.widgets.inactive.fg_stroke.color = color(&theme.inactive.stroke.color);
        style.visuals.widgets.inactive.fg_stroke.width = theme.inactive.stroke.size;

        style.visuals.widgets.noninteractive.bg_fill = color(&theme.noninteractive.background);
        style.visuals.widgets.noninteractive.bg_stroke.color =
            color(&theme.noninteractive.border.color);
        style.visuals.widgets.noninteractive.bg_stroke.width = theme.noninteractive.border.size;
        style.visuals.widgets.noninteractive.fg_stroke.color =
            color(&theme.noninteractive.stroke.color);
        style.visuals.widgets.noninteractive.fg_stroke.width = theme.noninteractive.stroke.size;

        style.visuals.widgets.hovered.bg_fill = color(&theme.hovered.background);
        style.visuals.widgets.hovered.bg_stroke.color = color(&theme.hovered.border.color);
        style.visuals.widgets.hovered.bg_stroke.width = theme.hovered.border.size;
        style.visuals.widgets.hovered.fg_stroke.color = color(&theme.hovered.stroke.color);
        style.visuals.widgets.hovered.fg_stroke.width = theme.hovered.stroke.size;

        style.visuals.widgets.open.bg_fill = color(&theme.open.background);
        style.visuals.widgets.open.bg_stroke.color = color(&theme.open.border.color);
        style.visuals.widgets.open.bg_stroke.width = theme.open.border.size;
        style.visuals.widgets.open.fg_stroke.color = color(&theme.open.stroke.color);
        style.visuals.widgets.open.fg_stroke.width = theme.open.stroke.size;

        egui_context.ctx_mut().set_style(style);
    }
}
