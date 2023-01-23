use dworkspace::repository::Repository;
use dworkspace_codebase::{event::Event, user::UserId};

pub struct InMemoryRepository {
    user_id: UserId,
    pub entries: Vec<Event>,
}

impl InMemoryRepository {
    pub fn new(user_id: UserId) -> Self {
        Self {
            user_id,
            entries: Vec::new(),
        }
    }
}

impl Repository for InMemoryRepository {
    fn poll(&mut self) -> Vec<Event> {
        self.entries.drain(..).collect()
    }

    fn commit(&mut self, event: Event) {
        self.entries.push(event);
    }
}
