use crate::{dprocess::DProcessId, status::DProcessStatus};

use super::VmRef;

impl<'a> VmRef<'a> {
    pub fn notify_status(&self, dprocess_id: &DProcessId, status: &DProcessStatus) {
        self.lock_migration_logic()
            .notify_status(dprocess_id, status);
    }
}
