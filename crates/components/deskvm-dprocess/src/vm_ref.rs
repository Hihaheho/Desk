use std::{collections::HashMap, ops::Deref, sync::Arc};

use parking_lot::RwLock;

use crate::{
    dprocess::{DProcess, DProcessId},
    dprocess_manifest::DProcessManifest,
    name_registry::NameRegistry,
    processor::ProcessorWithScheduler,
};

#[derive(Clone, Copy)]
// Fields must be private to prevent deadlocks.
pub struct VmRef<'a> {
    dprocesses: &'a RwLock<HashMap<DProcessId, Arc<DProcess>>>,
    processors: &'a RwLock<Vec<ProcessorWithScheduler>>,
    name_registry: &'a RwLock<NameRegistry>,
}

impl<'a> VmRef<'a> {
    pub fn read_dprocesses(&self) -> impl Deref<Target = HashMap<DProcessId, Arc<DProcess>>> + '_ {
        self.dprocesses.read()
    }

    pub fn read_processors(&self) -> impl Deref<Target = Vec<ProcessorWithScheduler>> + '_ {
        self.processors.read()
    }

    pub fn read_name_registry(&self) -> impl Deref<Target = NameRegistry> + '_ {
        self.name_registry.read()
    }
}

impl<'a> VmRef<'a> {
    pub fn spawn(&self, manifest: DProcessManifest) {
        todo!()
    }
}
