use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum DeskSystem {
    HandlingWidgetEvents,
    HandlingOperations,
    Shell,
    RenderingWidgets,
    PrePhysics,
}

#[derive(SystemLabel, PartialEq, Eq, Debug, Hash, Clone)]
pub enum ShellSystem {
    Add,
    UpdateComponent,
    UpdateWidget,
    Render,
}
