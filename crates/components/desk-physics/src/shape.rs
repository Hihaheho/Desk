use bevy_ecs::prelude::Component;

#[derive(Default, Component, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Shape {
    #[default]
    Blank,
    Rect {
        width: f32,
        height: f32,
    },
}
