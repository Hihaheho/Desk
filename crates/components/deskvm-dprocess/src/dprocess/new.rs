use std::collections::HashMap;

use parking_lot::RwLock;

use crate::{
    dprocess_manifest::DProcessManifest, flags::DProcessFlags,
    processor_attachment::ProcessorAttachment, status::DProcessStatus,
};

use super::DProcess;

impl DProcess {
    pub fn new(manifest: &DProcessManifest) -> Self {
        Self {
            interpreter: RwLock::new(manifest.interpreter_builder.build()),
            metas: RwLock::new(manifest.metas.clone()),
            effect_handlers: RwLock::new(manifest.effect_handlers.clone()),
            status: RwLock::new(DProcessStatus::Running),
            mailbox: RwLock::new(HashMap::new()),
            processor_attachment: RwLock::new(ProcessorAttachment::Detached),
            kv: RwLock::new(HashMap::new()),
            flags: RwLock::new(DProcessFlags::default()),
            timers: RwLock::new(HashMap::new()),
        }
    }
}
