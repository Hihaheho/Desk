mod get_dprocess;
mod get_processor;
mod notify_status;
mod pubsub;
mod read_locks;
mod register;
mod spawn;
mod write_locks;

use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use parking_lot::RwLock;

use crate::{
    dprocess::{DProcess, DProcessId},
    migration_logic::MigrationLogic,
    name_registry::NameRegistry,
    processor::{ProcessorName, ProcessorWithScheduler},
    status_update::StatusUpdate,
};

#[derive(Clone, Copy)]
// Fields must be private to prevent deadlocks.
pub struct VmRef<'a> {
    dprocesses: &'a RwLock<HashMap<DProcessId, Arc<DProcess>>>,
    processors: &'a RwLock<BTreeMap<ProcessorName, Arc<ProcessorWithScheduler>>>,
    name_registry: &'a RwLock<NameRegistry>,
    migration_logic: &'a RwLock<Box<dyn MigrationLogic>>,
    status_update: &'a RwLock<Vec<StatusUpdate>>,
}

impl<'a> VmRef<'a> {
    pub fn new(
        dprocesses: &'a RwLock<HashMap<DProcessId, Arc<DProcess>>>,
        processors: &'a RwLock<BTreeMap<ProcessorName, Arc<ProcessorWithScheduler>>>,
        name_registry: &'a RwLock<NameRegistry>,
        migration_logic: &'a RwLock<Box<dyn MigrationLogic>>,
        status_update: &'a RwLock<Vec<StatusUpdate>>,
    ) -> Self {
        Self {
            dprocesses,
            processors,
            name_registry,
            migration_logic,
            status_update,
        }
    }
}
