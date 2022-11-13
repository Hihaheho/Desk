use crate::{
    dprocess::DProcessId, processor::ProcessorName, processor_attachment::ProcessorAttachment,
    status_update::StatusUpdate, vm_ref::VmRef,
};

// TODO: This name should be more descriptive.
/// This trait implements how VM attaches a process to a processor.
///
/// Influenced by the Migration Logic of Erlang VM's scheduler.
/// Implementation never fails.
pub trait MigrationLogic: std::fmt::Debug {
    /// DeskVM completely respects the suggestions.
    fn suggest_migration<'a>(&mut self, vm: VmRef) -> Vec<MigrateSuggestion>;

    /// DeskVM calls this method when a new d-process is created.
    fn notify_new_dprocess(&mut self, dprocess_id: &DProcessId);

    /// DeskVM calls this method when a d-process is deleted.
    fn notify_deleted_dprocess(&mut self, dprocess_id: &DProcessId);

    /// DeskVM calls this method when a status of a d-process is updated.
    ///
    /// DeskVM does not calls this method for d-process creation.
    fn notify_status(&mut self, status_update: &StatusUpdate);

    /// DeskVM calls this method when a new processor is created.
    fn notify_new_processor(&mut self, processor_name: &ProcessorName);

    /// DeskVM calls this method when a processor is deleted.
    fn notify_deleted_processor(&mut self, processor_name: &ProcessorName);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MigrateSuggestion {
    pub process_id: DProcessId,
    pub to: ProcessorAttachment,
}
