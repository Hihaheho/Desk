use std::sync::Arc;

use dprocess::{
    dprocess::{DProcess, DProcessId},
    dprocess_manifest::DProcessManifest,
};

use super::DeskVm;

impl DeskVm {
    pub fn spawn(&self, manifest: &DProcessManifest) -> DProcessId {
        let process = Arc::new(DProcess::new(manifest));
        self.dprocesses
            .write()
            .insert(process.id.clone(), process.clone());
        self.migration_logic
            .write()
            .notify_new_dprocess(&process.id);
        process.id.clone()
    }

    pub fn delete_dprocess(&self, id: &DProcessId) {
        if self.dprocesses.write().remove(id).is_some() {
            self.migration_logic.write().notify_deleted_dprocess(id);
        }
    }
}
