use deskc_ids::UserId;

use crate::event::{Event, EventEntry};

pub trait Repository {
    fn poll(&mut self) -> Vec<EventEntry>;
    fn commit(&mut self, log: Event);
    fn add_owner(&mut self, user_id: UserId);
    fn remove_owner(&mut self, user_id: UserId);
}
