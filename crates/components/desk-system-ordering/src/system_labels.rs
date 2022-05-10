use bevy::prelude::*;
use derivative::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum DeskSystem {
    UpdateStatesToLatest,
    Shell,
    HandleOperations,
    PrePhysics,
}

#[derive(SystemLabel, PartialEq, Eq, Debug, Hash, Clone)]
pub enum ShellSystem {
    Add,
    UpdateComponent,
    UpdateWidget,
    Render,
    HandleEvents,
}
