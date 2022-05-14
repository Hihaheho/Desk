use crate::ctx::Ctx;
use uuid::Uuid;

pub trait Widget<T> {
    fn render<'a>(&mut self, ctx: &Ctx<'a, T>);
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct WidgetId(pub Uuid);

impl WidgetId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}
