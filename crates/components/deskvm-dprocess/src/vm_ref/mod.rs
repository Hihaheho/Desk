mod get_dprocess;
mod notify_status;
mod pubsub;
mod read_locks;
mod register;
mod spawn;
mod write_locks;

use std::{collections::HashMap, sync::Arc};

use parking_lot::RwLock;

use crate::{
    dprocess::{DProcess, DProcessId},
    migration_logic::MigrationLogic,
    name_registry::NameRegistry,
    processor::ProcessorWithScheduler,
};

#[derive(Clone, Copy)]
// Fields must be private to prevent deadlocks.
pub struct VmRef<'a> {
    dprocesses: &'a RwLock<HashMap<DProcessId, Arc<DProcess>>>,
    processors: &'a RwLock<Vec<ProcessorWithScheduler>>,
    name_registry: &'a RwLock<NameRegistry>,
    migration_logic: &'a RwLock<Box<dyn MigrationLogic>>,
}

impl<'a> VmRef<'a> {
    pub fn new(
        dprocesses: &'a RwLock<HashMap<DProcessId, Arc<DProcess>>>,
        processors: &'a RwLock<Vec<ProcessorWithScheduler>>,
        name_registry: &'a RwLock<NameRegistry>,
        migration_logic: &'a RwLock<Box<dyn MigrationLogic>>,
    ) -> Self {
        Self {
            dprocesses,
            processors,
            name_registry,
            migration_logic,
        }
    }
}
