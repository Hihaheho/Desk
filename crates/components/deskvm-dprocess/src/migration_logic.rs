use crate::{dprocess::DProcessId, processor_attachment::ProcessorAttachment, vm_ref::VmRef};

// TODO: This name should be more descriptive.
/// This trait implements how VM attaches a process to a processor.
///
/// Influenced by the Migration Logic of Erlang VM's scheduler.
/// Implementation never fails.
pub trait MigrationLogic: std::fmt::Debug {
    /// DeskVM respects the suggestions in best-effort.
    fn suggest_migration<'a>(&mut self, vm: &'a VmRef) -> Vec<MigrateSuggestion>;
}

pub struct MigrateSuggestion {
    pub process_id: DProcessId,
    pub to: ProcessorAttachment,
}
