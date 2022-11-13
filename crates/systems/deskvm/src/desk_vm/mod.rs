mod processors;
mod read_locks;
mod reduce;
mod run_migration_logic;
mod dprocesses;
mod status_updates;

use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use dprocess::{
    dprocess::{DProcess, DProcessId},
    migration_logic::MigrationLogic,
    name_registry::NameRegistry,
    processor::{ProcessorName, ProcessorWithScheduler},
    status_update::StatusUpdate,
    vm_ref::VmRef,
};
use parking_lot::RwLock;

#[derive(Debug)]
/// Influenced by Erlang VM but this is not tight-coupled with any interpreter of Desk-lang.
///
/// It allows any interpreter or executable binaries to be managed as a d-process in DeskVM.
/// For example, you can run a sandboxed DeskVM in a DeskVM (DeskVM on DeskVM).
// Fields must be private to prevent deadlocks and invalid access.
pub struct DeskVm {
    dprocesses: RwLock<HashMap<DProcessId, Arc<DProcess>>>,
    processors: RwLock<BTreeMap<ProcessorName, Arc<ProcessorWithScheduler>>>,
    migration_logic: RwLock<Box<dyn MigrationLogic>>,
    name_registry: RwLock<NameRegistry>,
    status_updates: RwLock<Vec<StatusUpdate>>,
}

impl DeskVm {
    pub fn new(migration_logic: impl MigrationLogic + 'static) -> Self {
        Self {
            dprocesses: Default::default(),
            processors: Default::default(),
            migration_logic: RwLock::new(Box::new(migration_logic)),
            name_registry: Default::default(),
            status_updates: Default::default(),
        }
    }

    pub fn vm_ref(&self) -> VmRef {
        VmRef::new(
            &self.dprocesses,
            &self.processors,
            &self.name_registry,
            &self.migration_logic,
            &self.status_updates,
        )
    }
}

#[cfg(test)]
mod tests {
    // TODO: Write tests. (It is hard to test because mry lacks features.)
}
