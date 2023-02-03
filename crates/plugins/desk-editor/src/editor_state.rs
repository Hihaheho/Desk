use std::collections::{BTreeMap, BTreeSet};

use deskc_ids::NodeId;
use dworkspace::{
    prelude::{Event, EventPayload, OperandPatch, Projection},
    state::State,
};
use egui::Pos2;

#[derive(Default)]
pub struct NodeState {
    pub line_split: bool,
    pub editing_text: Option<String>,
}

#[derive(Debug, Copy, PartialEq, Eq, Clone)]
pub struct WordCursor {
    pub node_id: NodeId,
    pub offset: u16,
}

#[derive(Debug, Copy, PartialEq, Eq, Clone)]
pub struct NextPos {
    pub pos: Pos2,
    pub search: WordSearch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordSearch {
    Forward,
    Backward,
    Nearest,
}

#[derive(Default)]
pub struct EditorState {
    pub selected_nodes: BTreeSet<NodeId>,
    pub node_states: BTreeMap<NodeId, NodeState>,
    /// Key is a top-level node.
    pub ephemeral_events: BTreeMap<NodeId, Vec<Event>>,
    /// Relative to the top-left corner of the source code.
    pub word_cursor: Option<WordCursor>,
    pub next_pos: Option<NextPos>,
    pub hovered_word: Option<WordCursor>,
}

impl State for EditorState {
    fn handle_event(&mut self, _snapshot: &Projection, event: &Event) {
        match &event.payload {
            EventPayload::RemoveNode { node_id } => {
                self.selected_nodes.remove(node_id);
                self.node_states.remove(node_id);
                self.ephemeral_events.remove(node_id);
            }
            EventPayload::PatchOperand {
                node_id: _,
                patch:
                    OperandPatch::Insert {
                        node_id: operand, ..
                    },
            } => {
                // The operand is no longer top-level.
                self.ephemeral_events.remove(operand);
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
