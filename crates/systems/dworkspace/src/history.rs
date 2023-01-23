use components::{
    event::{Event, EventPayload},
    projection::Projection,
};

use crate::state::State;

#[derive(Default)]
pub struct History {}

impl State for History {
    fn handle_event(&mut self, projection: &Projection, event: &Event) {
        // match &event.payload {
        //     EventPayload::AddOwner { user_id } => todo!(),
        //     EventPayload::RemoveOwner { user_id } => todo!(),
        //     EventPayload::UpdateSpaceRules { rules } => todo!(),
        //     EventPayload::CreateNode { node_id, content } => todo!(),
        //     EventPayload::RemoveNode { node_id } => todo!(),
        //     EventPayload::PatchContent { node_id, patch } => todo!(),
        //     EventPayload::PatchOperand { node_id, patch } => todo!(),
        //     EventPayload::PatchAttribute { node_id, patch } => todo!(),
        //     EventPayload::UpdateNodeRules { node_id, rules } => todo!(),
        //     EventPayload::UpdateOperandRules { node_id, rules } => todo!(),
        //     EventPayload::AddSnapshot { index, snapshot } => todo!(),
        // }
    }
}

#[cfg(test)]
mod tests {
    use components::{event::EventId, user::UserId};

    use super::*;

    fn e(payload: EventPayload) -> Event {
        Event {
            id: EventId::new(),
            user_id: UserId::new(),
            payload,
        }
    }

    #[test]
    fn test_history() {
        let mut history = History::default();
        let mut projection = Projection::default();
        let user_id = UserId::new();
        let event = e(EventPayload::AddOwner { user_id });
        history.handle_event(&projection, &event);
        // TODO
    }
}
