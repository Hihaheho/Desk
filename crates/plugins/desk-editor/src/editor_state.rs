use std::collections::BTreeMap;

use deskc_ids::NodeId;
use dworkspace::{
    prelude::{Event, EventPayload, Projection},
    state::State,
};

#[derive(Default)]
pub struct NodeState {
    pub line_split: bool,
    pub editing_text: Option<String>,
}

#[derive(Default)]
pub struct EditorState {
    pub selected_node: Option<NodeId>,
    pub selected_operand: Option<usize>,
    pub node_states: BTreeMap<NodeId, NodeState>,
}

impl State for EditorState {
    fn handle_event(&mut self, _snapshot: &Projection, event: &Event) {
        match &event.payload {
            EventPayload::RemoveNode { node_id } => {
                self.selected_node = self.selected_node.filter(|id| id != node_id);
            }
            _ => {}
        }
    }
}

impl EditorState {
    pub fn node_mut(&mut self, node_id: NodeId) -> &mut NodeState {
        self.node_states.entry(node_id).or_default()
    }
}
