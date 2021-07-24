pub trait EventHandler: Send + Sync + 'static {
    type Context: Send + Sync + 'static + std::fmt::Debug;
    type Events: Send + Sync + 'static + std::fmt::Debug;
    type Output: Send + Sync + 'static + std::fmt::Debug;

    fn handle(&self, context: &Self::Context, events: &Self::Events) -> Self::Output;
}
