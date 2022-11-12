use std::{collections::HashMap, ops::Deref, sync::Arc};

use crate::{
    dprocess::{DProcess, DProcessId},
    name_registry::NameRegistry,
    processor::ProcessorWithScheduler,
};

use super::VmRef;

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
