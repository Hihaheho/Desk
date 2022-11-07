use components::{
    event::{Event, EventEntry},
    user::UserId,
};

pub trait Repository {
    fn poll(&mut self) -> Vec<EventEntry>;
    fn commit(&mut self, event: Event);
    fn add_owner(&mut self, user_id: UserId);
    fn remove_owner(&mut self, user_id: UserId);
}

#[cfg(test)]
#[mry::mry]
#[derive(Default)]
pub struct TestRepository {}

#[cfg(test)]
#[mry::mry]
impl Repository for TestRepository {
    fn poll(&mut self) -> Vec<EventEntry> {
        panic!()
    }
    fn commit(&mut self, log: Event) {
        panic!()
    }
    fn add_owner(&mut self, user_id: UserId) {
        panic!()
    }
    fn remove_owner(&mut self, user_id: UserId) {
        panic!()
    }
}
