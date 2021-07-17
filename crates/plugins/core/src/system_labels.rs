use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum DeskSystem {
    HandlingWidgetEvents,
    HandlingOperations,
    Shell,
    RenderingWidgets,
    PrePhysics,
}
