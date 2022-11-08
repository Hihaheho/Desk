use deskc_ids::NodeId;
use dworkspace::state::State;

#[derive(Default)]
pub struct EditorState {
    // For prototype
    pub child_addition_target: Option<NodeId>,
}

impl State for EditorState {
    fn handle_event(
        &mut self,
        _snapshot: &dworkspace_codebase::snapshot::Snapshot,
        _log: &dworkspace_codebase::event::Event,
    ) {
    }
}
