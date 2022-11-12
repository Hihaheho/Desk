use std::sync::Arc;

use crate::{dprocess::DProcess, dprocess_manifest::DProcessManifest};

use super::VmRef;

impl<'a> VmRef<'a> {
    pub fn spawn(&self, manifest: &DProcessManifest) {
        let dprocess = DProcess::new(manifest);
        let dprocess_id = dprocess.id.clone();
        self.lock_dprocesses()
            .insert(dprocess_id.clone(), Arc::new(dprocess));
        self.lock_migration_logic().notify_new_process(dprocess_id);
    }
}
