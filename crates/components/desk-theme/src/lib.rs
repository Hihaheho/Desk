pub mod colorscheme;

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
    pub window_background: Color,
    pub window_corner_radius: f32,
    pub window_shadow: Shadow,
    pub extreme_background: Color,
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

#[derive(Reflect, Serialize, Deserialize, Clone)]
pub struct EditorStyle {
    // How many spaces for each indentation level
    pub indent_width: u8,
    pub indent_guide: IndentGuide,
    pub line_spacing: f32,
    pub line_number: bool,
}

impl Default for EditorStyle {
    fn default() -> Self {
        Self {
            indent_width: 2,
            indent_guide: Default::default(),
            line_spacing: 0.2,
            line_number: true,
        }
    }
}

#[derive(Reflect, Serialize, Deserialize, Clone)]
pub enum IndentGuide {
    None,
    SingleColorLine { size: f32 },
}

impl Default for IndentGuide {
    fn default() -> Self {
        Self::SingleColorLine { size: 0.2 }
    }
}
