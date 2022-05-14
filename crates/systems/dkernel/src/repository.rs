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
