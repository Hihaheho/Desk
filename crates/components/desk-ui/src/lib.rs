use bevy_ecs::prelude::Component;
use uuid::Uuid;

#[derive(Component, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WidgetId(pub Uuid);
