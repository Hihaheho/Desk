use std::{
    collections::{BTreeMap, HashMap},
    ops::Deref,
    sync::Arc,
};

use dprocess::{
    dprocess::{DProcess, DProcessId},
    name_registry::NameRegistry,
    processor::{ProcessorName, ProcessorWithScheduler},
};

use super::DeskVm;

impl DeskVm {
    pub fn read_dprocesses(&self) -> impl Deref<Target = HashMap<DProcessId, Arc<DProcess>>> + '_ {
        self.dprocesses.read()
    }

    pub fn read_processors(
        &self,
    ) -> impl Deref<Target = BTreeMap<ProcessorName, Arc<ProcessorWithScheduler>>> + '_ {
        self.processors.read()
    }

    pub fn read_name_registry(&self) -> impl Deref<Target = NameRegistry> + '_ {
        self.name_registry.read()
    }
}
