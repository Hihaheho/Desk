use dprocess::{dprocess::DProcessId, processor_attachment::ProcessorAttachment};

use super::DeskVm;

impl DeskVm {
    pub fn run_migration_logic(&self) {
        for suggestion in self
            .migration_logic
            .write()
            .suggest_migration(self.vm_ref())
        {
            self.migrate(suggestion.process_id, suggestion.to);
        }
    }

    pub fn migrate(&self, process_id: DProcessId, to: ProcessorAttachment) {
        if let Some(process) = self.dprocesses.read().get(&process_id) {
            // Detach from current processor
            match &*process.read_processor_attachment() {
                ProcessorAttachment::Attached(processor_name) => {
                    if let Some(processor) = self.processors.read().get(processor_name) {
                        processor.scheduler.write().detach(&process.id);
                    }
                }
                ProcessorAttachment::Detached => {}
            }
            // Attach to the new processor
            match &to {
                ProcessorAttachment::Attached(processor_name) => {
                    if let Some(processor) = self.processors.read().get(processor_name) {
                        processor.scheduler.write().attach(process.clone());
                    }
                }
                ProcessorAttachment::Detached => {}
            }
            process.update_processor_attachment(to);
        }
    }
}
