pub mod backend;
pub mod component;
pub mod event;

use bevy_math::Vec2;
use language::code::path::NodePath;
use protocol::card_id::CardId;

use crate::shape::Shape;

use self::{component::Component, event::WidgetEvent};

#[derive(Clone, Debug)]

pub enum Action {
    UpdateString { value: String, target: Target },
}

#[derive(Clone, Debug)]
pub enum Target {
    Terminal,
    Card {
        card_id: CardId,
        node_path: NodePath,
    },
}

#[derive(Clone, Debug, Hash)]
pub struct WidgetId(pub String);

impl<T: Into<String>> From<T> for WidgetId {
    fn from(from: T) -> Self {
        Self(from.into())
    }
}

pub trait WidgetSystem {
    fn render(&self) -> Widget;
    fn update(&mut self, events: dyn Iterator<Item = WidgetEvent>) -> WidgetEvent;
}

#[derive(Clone, Debug)]
pub struct Widget {
    pub id: WidgetId,
    pub position: Vec2,
    pub shape: Option<Shape>,
    pub component: Component,
}
