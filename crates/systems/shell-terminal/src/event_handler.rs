use physics::{
    event_handler::EventHandler,
    widget::event::{WidgetEvent, WidgetEvents},
};
use terminal::{terminal::Terminal, TerminalOperation, TerminalOperations};

#[derive(Debug, Default)]
pub struct TerminalWidgetEventHandler;

impl EventHandler for TerminalWidgetEventHandler {
    type Context = Terminal;
    type Events = WidgetEvents;
    type Output = TerminalOperations;

    fn handle(&self, _context: &Self::Context, events: &Self::Events) -> Self::Output {
        events
            .iter()
            .flat_map(|event| {
                use WidgetEvent::*;
                match event {
                    UpdateString { id: _, value } => {
                        vec![TerminalOperation::UpdateCommandInput(value.clone())]
                    }
                    _ => vec![],
                }
                .into_iter()
            })
            .into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn update_command_input() {
        let handler = TerminalWidgetEventHandler;
        let terminal = Default::default();
        assert_eq!(
            handler.handle(
                &terminal,
                &vec![WidgetEvent::UpdateString {
                    id: "any".into(),
                    value: "new_value".into()
                }]
                .into()
            ),
            vec![TerminalOperation::UpdateCommandInput("new_value".into())]
                .into_iter()
                .into()
        )
    }
}
