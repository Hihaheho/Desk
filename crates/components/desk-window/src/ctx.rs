use bevy_math::Vec2;
use dworkspace::Workspace;
use dworkspace_codebase::event::Event;

pub struct Ctx<'a, Backend> {
    pub events: Vec<Event>,
    pub workspace: &'a mut Workspace,
    backend: Backend,
    pub offset: &'a mut Vec2,
}

impl<'a, T: Clone> Ctx<'a, T> {
    pub fn new(workspace: &'a mut Workspace, backend: T, offset: &'a mut Vec2) -> Ctx<'a, T> {
        Self {
            events: Vec::new(),
            workspace,
            backend,
            offset,
        }
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn backend(&self) -> T {
        self.backend.clone()
    }
}
