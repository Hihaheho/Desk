use crate::{
    dprocess::DProcess, processor_attachment::ProcessorAttachment, status::DProcessStatus,
    status_update::StatusUpdate,
};

use super::VmRef;

impl<'a> VmRef<'a> {
    pub fn notify_status(&self, dprocess: &DProcess, status: &DProcessStatus) {
        let status_update = StatusUpdate {
            dprocess_id: dprocess.id.clone(),
            status: status.clone(),
        };
        // Notify to migration logics
        self.lock_migration_logic().notify_status(&status_update);
        // Notify to the attached scheduler
        if let ProcessorAttachment::Attached(processor) = &*dprocess.read_processor_attachment() {
            if let Some(processor) = self.get_processor(processor) {
                processor.scheduler.write().notify_status(&status_update);
            }
        }
        // Push to the status queue
        self.lock_status_update().push(status_update);
    }
}
