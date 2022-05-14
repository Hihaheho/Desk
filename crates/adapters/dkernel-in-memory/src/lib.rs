use dkernel::repository::Repository;
use dkernel_components::{
    event::{Event, EventEntry},
    user::UserId,
};

pub struct InMemoryRepository {
    user_id: UserId,
    index: usize,
    pub entries: Vec<EventEntry>,
}

impl InMemoryRepository {
    pub fn new(user_id: UserId) -> Self {
        Self {
            user_id,
            index: 0,
            entries: Vec::new(),
        }
    }
}

impl Repository for InMemoryRepository {
    fn poll(&mut self) -> Vec<EventEntry> {
        self.entries.drain(..).collect()
    }

    fn commit(&mut self, event: Event) {
        self.entries.push(EventEntry {
            index: self.index,
            user_id: self.user_id.clone(),
            event,
        });
        self.index += 1;
    }

    fn add_owner(&mut self, _user_id: UserId) {
        todo!()
    }

    fn remove_owner(&mut self, _user_id: UserId) {
        todo!()
    }
}
