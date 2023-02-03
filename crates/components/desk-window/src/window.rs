use std::collections::HashMap;

use bevy_ecs::prelude::Component;
use bevy_math::Vec2;
use uuid::Uuid;

use crate::widget::{Widget, WidgetId};

#[derive(Component)]
pub struct DefaultWindow;

#[derive(Component, Default)]
pub struct Window<T> {
    pub offset: Vec2,
    widgets: HashMap<WidgetId, Box<dyn Widget<T> + Send + Sync + 'static>>,
}

#[derive(Component, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WindowId(pub Uuid);

impl<T> Window<T> {
    pub fn drain_widgets(&mut self) -> Vec<Box<dyn Widget<T> + Send + Sync + 'static>> {
        self.widgets.drain().map(|(_, widget)| widget).collect()
    }

    pub fn add_widget<W: Widget<T> + Send + Sync + 'static>(
        &mut self,
        widget_id: WidgetId,
        widget: W,
    ) {
        self.widgets.insert(widget_id, Box::new(widget));
    }
}
