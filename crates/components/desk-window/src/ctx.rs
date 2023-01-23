use dworkspace::Workspace;
use dworkspace_codebase::event::Event;

pub struct Ctx<'a, Backend> {
    events: Vec<Event>,
    pub workspace: &'a mut Workspace,
    backend: Backend,
}

impl<'a, T: Clone> Ctx<'a, T> {
    pub fn new(kernel: &'a mut Workspace, backend: T) -> Self {
        Self {
            events: Vec::new(),
            workspace: kernel,
            backend,
        }
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn events(self) -> Vec<Event> {
        self.events
    }

    pub fn backend(&self) -> T {
        self.backend.clone()
    }
}
