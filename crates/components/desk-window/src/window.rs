use std::collections::HashMap;

use bevy_ecs::prelude::Component;
use uuid::Uuid;

use crate::{
    ctx::Ctx,
    widget::{Widget, WidgetId},
};

#[derive(Component)]
pub struct DefaultWindow;

#[derive(Component, Default)]
pub struct Window<T> {
    widgets: HashMap<WidgetId, Box<dyn Widget<T> + Send + Sync + 'static>>,
}

#[derive(Component, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WindowId(pub Uuid);

impl<T> Window<T> {
    pub fn render(&mut self, ctx: &mut Ctx<T>) {
        for (_id, mut widget) in self.widgets.drain() {
            widget.render(ctx);
        }
    }

    pub fn add_widget<W: Widget<T> + Send + Sync + 'static>(
        &mut self,
        widget_id: WidgetId,
        widget: W,
    ) {
        self.widgets.insert(widget_id, Box::new(widget));
    }
}
