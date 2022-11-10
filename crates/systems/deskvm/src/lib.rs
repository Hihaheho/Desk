use std::{collections::HashMap, sync::Arc, time::Duration};

use dprocess::{
    dprocess::{DProcess, DProcessId},
    migration_logic::MigrationLogic,
    name_registry::NameRegistry,
    processor::ProcessorWithScheduler,
    processor_attachment::ProcessorAttachment,
    vm_ref::VmRef,
    vm_output::VmOutputs,
};
use parking_lot::RwLock;

#[derive(Debug)]
/// Influenced by Erlang VM but this is not tight-coupled with any interpreter of Desk-lang.
///
/// It allows any interpreter or executable binaries to be managed as a d-process in DeskVM.
/// For example, you can run a sandboxed DeskVM in a DeskVM (DeskVM on DeskVM).
pub struct DeskVm {
    // uses Arc to make the process ownable by a processor.
    pub dprocesses: RwLock<HashMap<DProcessId, Arc<DProcess>>>,
    pub processors: RwLock<Vec<ProcessorWithScheduler>>,
    pub process_attacher: RwLock<Box<dyn MigrationLogic>>,
    pub name_registry: RwLock<NameRegistry>,
}

impl DeskVm {
    // VM never fails.
    /// An API for single-threaded platform such as the Web or realtime application like games.
    pub fn reduce(&mut self, target_duration: &Duration) -> VmOutputs {
        // This is a single threaded version.
        let divided_duration = target_duration / self.processors.read().len() as u32;
        VmOutputs::merge(self.processors.read().iter().map(|pws| {
            pws.scheduler
                .write()
                .reduce(&pws.processor.read(), divided_duration)
        }))
    }

    pub fn run_migration_logic(&self) {
        for suggestion in self
            .process_attacher
            .write()
            .suggest_migration(&self.vm_info())
        {
            self.migrate(suggestion.process_id, suggestion.to);
        }
    }

    pub fn migrate(&self, process_id: DProcessId, to: ProcessorAttachment) {
        if let Some(process) = self.dprocesses.read().get(&process_id) {
            match to {
                ProcessorAttachment::Attached(processor_id) => {
                    if let Some(processor) = self.processors.read().get(processor_id.0) {
                        processor
                            .scheduler
                            .write()
                            .attach(process_id, process.clone());
                    }
                }
                ProcessorAttachment::Detached => todo!(),
            }
        }
    }

    pub fn vm_info(&self) -> VmRef {
        VmRef {
            dprocesses: &self.dprocesses,
            processors: &self.processors,
            name_registry: &self.name_registry,
        }
    }
}

#[cfg(test)]
mod tests {
    // TODO: Write tests. (It is hard to test because mry lacks features.)
}
