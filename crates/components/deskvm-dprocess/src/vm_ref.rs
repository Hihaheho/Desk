use std::{collections::HashMap, ops::Deref, sync::Arc};

use parking_lot::RwLock;
use types::Type;

use crate::{
    dprocess::{DProcess, DProcessId},
    dprocess_manifest::DProcessManifest,
    flags::DProcessFlags,
    name_registry::NameRegistry,
    processor::ProcessorWithScheduler,
    value::Value,
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

    pub fn subscribe(&self, dprocess_id: DProcessId, ty: Type) {
        todo!()
    }

    pub fn publish(&self, ty: Type, value: Value) {
        todo!()
    }

    pub fn get_flags(&self, dprocess_id: &DProcessId) -> Option<&DProcessFlags> {
        todo!()
    }

    pub fn get_mut_flags(&self, dprocess_id: &DProcessId) -> Option<&mut DProcessFlags> {
        todo!()
    }

    pub fn monitor(&self, dprocess_id: DProcessId, target: DProcessId) {
        todo!()
    }

    pub fn demonitor(&self, dprocess_id: DProcessId, target: DProcessId) {
        todo!()
    }

    pub fn link(&self, dprocess_id: DProcessId, target: DProcessId) {
        todo!()
    }

    pub fn unlink(&self, dprocess_id: DProcessId, target: DProcessId) {
        todo!()
    }

    pub fn register(&self, name: String, dprocess_id: DProcessId) {
        todo!()
    }

    pub fn unregister(&self, name: String) {
        todo!()
    }

    pub fn whereis(&self, name: String) -> Option<DProcessId> {
        todo!()
    }

    pub fn halt_dprocess(&self, dprocess_id: DProcessId, ty: Type, reason: Value) {
        todo!()
    }
}
