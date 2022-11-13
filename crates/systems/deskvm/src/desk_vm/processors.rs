use std::sync::Arc;

use dprocess::processor::{ProcessorManifest, ProcessorName, ProcessorWithScheduler};

use super::DeskVm;

impl DeskVm {
    pub fn add_processor(&self, manifest: ProcessorManifest) {
        self.migration_logic
            .write()
            .notify_new_processor(&manifest.name);
        self.processors.write().insert(
            manifest.name.clone(),
            Arc::new(ProcessorWithScheduler::new(manifest)),
        );
    }

    pub fn delete_processor(&self, name: &ProcessorName) {
        if self.processors.write().remove(name).is_some() {
            self.migration_logic.write().notify_deleted_processor(name);
        }
    }
}
