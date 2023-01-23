use components::event::Event;

pub trait Repository {
    fn poll(&mut self) -> Vec<Event>;
    fn commit(&mut self, event: Event);
}

#[cfg(test)]
#[mry::mry]
#[derive(Default)]
pub struct TestRepository {}

#[cfg(test)]
#[mry::mry]
impl Repository for TestRepository {
    fn poll(&mut self) -> Vec<Event> {
        panic!()
    }
    fn commit(&mut self, log: Event) {
        panic!()
    }
}
