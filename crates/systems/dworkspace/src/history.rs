use std::collections::VecDeque;

use components::{
    event::{Event, EventPayload},
    projection::Projection,
};

use crate::state::State;

pub struct History {
    size: usize,
    undo_stack: VecDeque<EventPayload>,
}

impl History {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            undo_stack: VecDeque::new(),
        }
    }
    fn push(&mut self, payload: EventPayload) {
        self.undo_stack.push_back(payload);
        if self.undo_stack.len() > self.size {
            self.undo_stack.pop_front();
        }
    }
    pub fn is_empty(&self) -> bool {
        self.undo_stack.is_empty()
    }
    pub fn undo_all(&mut self) -> Vec<EventPayload> {
        self.undo_stack.drain(..).collect()
    }
}

impl State for History {
    fn handle_event(&mut self, projection: &Projection, event: &Event) {
        match &event.payload {
            EventPayload::AddOwner { user_id } => {}
            EventPayload::RemoveOwner { user_id } => {}
            EventPayload::UpdateSpaceRules { rules } => {}
            EventPayload::CreateNode { node_id, content } => {}
            EventPayload::RemoveNode { node_id } => {}
            EventPayload::PatchContent { node_id, patch } => {}
            EventPayload::PatchOperand { node_id, patch } => {}
            EventPayload::PatchAttribute { node_id, patch } => {}
            EventPayload::UpdateNodeRules { node_id, rules } => {}
            EventPayload::UpdateOperandRules { node_id, rules } => {}
            EventPayload::AddSnapshot { index, snapshot } => {}
        }
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
        let mut history = History::new(10);
        let mut projection = Projection::default();
        let user_id = UserId::new();
        let event = e(EventPayload::AddOwner { user_id });
        history.handle_event(&projection, &event);
        // assert_eq!(history.undo_stack, [
    }
}
