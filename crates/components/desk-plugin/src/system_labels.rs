use bevy_ecs::schedule::SystemLabel;

#[derive(SystemLabel, Clone)]
pub enum DeskSystem {
    UpdateWidget,
    RenderWidget,
    ProcessKernel,
    PrePhysics,
}

#[derive(SystemLabel, Clone)]
pub enum ShellSystem {
    Add,
    UpdateComponent,
    UpdateWidget,
    Render,
    HandleEvents,
}
