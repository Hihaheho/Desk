use parking_lot::RwLock;

use crate::dprocess_manifest::DProcessManifest;

use super::{DProcess, DProcessId};

impl DProcess {
    pub fn new(manifest: &DProcessManifest) -> Self {
        Self {
            id: DProcessId::new(),
            interpreter: RwLock::new(manifest.interpreter_builder.build()),
            metas: RwLock::new(manifest.metas.clone()),
            effect_handlers: RwLock::new(manifest.effect_handlers.clone()),
            status: Default::default(),
            mailbox: Default::default(),
            processor_attachment: Default::default(),
            kv: Default::default(),
            flags: Default::default(),
            timers: Default::default(),
            monitors: Default::default(),
            links: Default::default(),
        }
    }
}
