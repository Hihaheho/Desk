use core::ShellSystem;

use bevy::{prelude::*, render::camera::Camera};
use egui_backend::{
    egui::{FontDefinitions, FontFamily},
    EguiBackend, EguiContext, EguiPlugin,
};
use physics::widget::{backend::Backends, component::Component, Widget, WidgetId};

const EGUI_BACKEND: &str = "egui";

pub struct EguiBackendPlugin;

impl Plugin for EguiBackendPlugin {
    fn build(&self, app: &mut bevy::app::AppBuilder) {
        app.add_plugin(EguiPlugin)
            .add_system(
                add_font
                    .system()
                    .with_run_criteria(bevy::ecs::schedule::RunOnce::default()),
            )
            .add_system(add_backend.system().before(ShellSystem::Render))
            .add_system(update_widget.system().label(ShellSystem::UpdateWidget));
    }
}

fn add_font(egui_context: ResMut<EguiContext>) {
    let ctx = egui_context.ctx();
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "mplus".to_owned(),
        std::borrow::Cow::Borrowed(include_bytes!(
            "../../../../assets/fonts/MPLUSRounded1c-Regular.ttf"
        )),
    );

    // Put my font first (highest priority):
    fonts
        .fonts_for_family
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "mplus".to_owned());

    ctx.set_fonts(fonts);
}

fn add_backend(mut backends: ResMut<Backends>, egui_context: ResMut<EguiContext>) {
    let backend = EguiBackend {
        ctx: egui_context.ctx().clone(),
    };
    backends.insert(EGUI_BACKEND, backend);
}

fn update_widget(
    mut commands: Commands,
    windows: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut query: Query<(
        Entity,
        Option<&mut Widget>,
        &WidgetId,
        &Component,
        &GlobalTransform,
    )>,
) {
    let (camera, camera_transform) = camera.single().unwrap();
    for (entity, widget, id, component, transform) in query.iter_mut() {
        if let Some(position) =
            camera.world_to_screen(&windows, camera_transform, transform.translation)
        {
            let new_widget = Widget {
                id: id.to_owned(),
                backend_id: EGUI_BACKEND.into(),
                component: component.to_owned(),
                position: position.to_owned().extend(0.0),
            };
            if let Some(mut widget) = widget {
                *widget = new_widget;
            } else {
                commands.entity(entity).insert(new_widget);
            }
        }
    }
}
