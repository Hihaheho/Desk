pub mod backend;
pub mod component;
pub mod event;

use bevy_math::Vec3;

use self::{component::Component, event::WidgetEvent};

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct WidgetId(pub String);

impl ToString for WidgetId {
    fn to_string(&self) -> String {
        self.0.to_owned()
    }
}

impl<T: Into<String>> From<T> for WidgetId {
    fn from(from: T) -> Self {
        Self(from.into())
    }
}

pub trait WidgetSystem {
    fn render(&self) -> Widget;
    fn update(&mut self, events: dyn Iterator<Item = WidgetEvent>) -> WidgetEvent;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Widget {
    pub id: WidgetId,
    pub position: Vec3,
    pub component: Component,
}
