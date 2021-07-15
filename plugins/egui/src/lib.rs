use core::DeskSystem;

use bevy::{prelude::*, render::camera::Camera};
use egui_backend::{EguiBackend, EguiContext, EguiPlugin};
use physics::{
    shape::Shape,
    widget::{backend::WidgetBackend, component::Component, Widget, WidgetId},
    Velocity,
};

struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut bevy::app::AppBuilder) {
        app.add_system(
            widget_rendering
                .system()
                .label(DeskSystem::RenderingWidgets)
                .before(DeskSystem::PrePhysics),
        );
    }
}

fn widget_rendering(
    windows: Res<Windows>,
    egui_context: ResMut<EguiContext>,
    time: Res<Time>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut query: Query<(
        &WidgetId,
        &Component,
        &mut Shape,
        &mut Velocity,
        &GlobalTransform,
    )>,
) {
    let (camera, camera_transform) = camera.single().unwrap();
    for (id, component, mut shape, mut velocity, transform) in query.iter_mut() {
        if let Some(position) =
            camera.world_to_screen(&windows, camera_transform, transform.translation)
        {
            let mut backend = EguiBackend {
                ctx: egui_context.ctx().clone(),
            };

            let response = backend.render(&Widget {
                id: id.to_owned(),
                component: component.to_owned(),
                position: position.to_owned().extend(0.0),
            });

            let new_shape = response.shape;
            if *shape != new_shape {
                *shape = dbg!(new_shape);
            }

            let new_velocity = (response.drag_delta / time.delta_seconds()).into();
            if *velocity != new_velocity {
                *velocity = new_velocity;
            }
        }
    }
}

pub struct EguiPlugins;

impl PluginGroup for EguiPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(CardPlugin).add(EguiPlugin);
    }
}
