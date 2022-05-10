use bevy_ecs::prelude::Component;

#[derive(Component, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Shape {
    Blank,
    Rect { width: f32, height: f32 },
}

impl Default for Shape {
    fn default() -> Self {
        Shape::Blank
    }
}
