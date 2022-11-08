use dworkspace::Workspace;
use dworkspace_codebase::event::Event;

pub struct Ctx<'a, Backend> {
    pub events: Vec<Event>,
    pub kernel: &'a mut Workspace,
    // this is not mut because of egui...
    pub backend: &'a Backend,
}

impl<'a, T> Ctx<'a, T> {
    pub fn new(kernel: &'a mut Workspace, backend: &'a T) -> Self {
        Self {
            events: Vec::new(),
            kernel,
            backend,
        }
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }
}
