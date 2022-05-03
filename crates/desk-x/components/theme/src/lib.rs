use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Reflect, Default, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Theme {
    /// hoverred interactive widget
    pub hovered: Widget,
    /// opened combo box
    pub open: Widget,
    /// non-interactive widget
    pub noninteractive: Widget,
    /// inactive widget
    pub inactive: Widget,
    /// active widget
    pub active: Widget,
    pub window_corner_radius: f32,
    pub window_shadow: Shadow,
}

#[derive(Reflect, Serialize, Deserialize, Clone)]
pub struct Stroke {
    pub color: Color,
    pub size: f32,
}

impl Default for Stroke {
    fn default() -> Self {
        Self {
            color: Color::BLACK,
            size: 1.0,
        }
    }
}

#[derive(Reflect, Default, Serialize, Deserialize)]
pub struct Shadow {
    pub color: Color,
    pub extrusion: f32,
}

#[derive(Reflect, Default, Serialize, Deserialize)]
pub struct Widget {
    pub background: Color,
    pub border: Stroke,
    pub stroke: Stroke,
}
