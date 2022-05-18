use deskc_ids::NodeId;
use dkernel::state::State;

#[derive(Default)]
pub struct EditorState {
    // For prototype
    pub child_addition_target: Option<NodeId>,
}

impl State for EditorState {
    fn handle_event(
        &mut self,
        _snapshot: &dkernel_components::snapshot::Snapshot,
        _log: &dkernel_components::event::Event,
    ) {
    }
}
