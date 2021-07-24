use language::code::operation::{CodeOperation, CodeOperations};
use physics::{
    event_handler::EventHandler,
    widget::{
        event::{WidgetEvent, WidgetEvents},
        Widget,
    },
};

pub struct CodeWidgetEventHandler;

impl EventHandler for CodeWidgetEventHandler {
    type Context = Widget;
    type Events = WidgetEvents;
    type Output = CodeOperations;

    fn handle(&self, widget: &Self::Context, events: &WidgetEvents) -> CodeOperations {
        events
            .iter()
            .flat_map(|event| handle_event(widget, event))
            .collect::<Vec<_>>()
            .into()
    }
}

fn handle_event(_widget: &Widget, event: &WidgetEvent) -> impl Iterator<Item = CodeOperation> {
    use WidgetEvent::*;
    match event {
        UpdateString { id: _id, value } => {
            // TODO handle id
            vec![CodeOperation::UpdateString(value.clone())].into_iter()
        }
        _ => vec![].into_iter(),
    }
}

#[cfg(test)]
mod test {
    use bevy_math::Vec3;
    use physics::widget::component::sugar as c;

    use super::*;

    #[test]
    fn updates_string() {
        let event_handler = CodeWidgetEventHandler;
        assert_eq!(
            event_handler.handle(
                &Widget {
                    id: "a".into(),
                    backend_id: "a".into(),
                    position: Vec3::new(0., 0., 0.),
                    component: c::input_string("id1", "a")
                },
                &vec![WidgetEvent::UpdateString {
                    id: "id1".into(),
                    value: "aa".into()
                }]
                .into()
            ),
            vec![CodeOperation::UpdateString("aa".into())].into()
        )
    }
}
