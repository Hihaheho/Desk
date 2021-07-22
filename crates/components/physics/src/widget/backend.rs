use std::{collections::HashMap, fmt::Debug};

use bevy_math::Vec2;

use crate::{shape::Shape, DragState};

use super::{event::WidgetEvents, Widget};

#[derive(Debug, Clone)]
pub struct RenderResponse {
    pub shape: Shape,
    pub events: WidgetEvents,
    pub drag_state: DragState,
    pub drag_delta: Vec2,
}

pub trait WidgetBackend {
    fn render(&mut self, widget: &Widget) -> RenderResponse;
}

type BoxedDynBackend = Box<dyn WidgetBackend + Send + Sync>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WidgetBackendId(pub String);

impl<T: Into<String>> From<T> for WidgetBackendId {
    fn from(from: T) -> Self {
        Self(from.into())
    }
}

pub struct Backends {
    backends: HashMap<WidgetBackendId, BoxedDynBackend>,
}

impl Backends {
    pub fn new() -> Self {
        Self {
            backends: HashMap::new(),
        }
    }

    pub fn insert<T: Into<WidgetBackendId>, B: WidgetBackend + Send + Sync + 'static>(
        &mut self,
        id: T,
        backend: B,
    ) {
        self.backends.insert(id.into(), Box::new(backend));
    }

    pub fn get_mut(&mut self, id: &WidgetBackendId) -> Option<&mut BoxedDynBackend> {
        self.backends.get_mut(id)
    }
}

impl Default for Backends {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    pub struct TestBackend;

    impl WidgetBackend for TestBackend {
        fn render(&mut self, _: &Widget) -> RenderResponse {
            todo!()
        }
    }

    #[test]
    fn set_backend() {
        let mut backends = Backends::new();
        backends.insert("primary", TestBackend);
        assert!(backends.backends.get(&"primary".into()).is_some());
    }

    #[test]
    fn get_backend() {
        let mut backends: HashMap<WidgetBackendId, BoxedDynBackend> = HashMap::new();
        backends.insert("primary".into(), Box::new(TestBackend));
        let mut backends = Backends { backends };
        assert!(backends.get_mut(&"primary".into()).is_some());
    }
}
