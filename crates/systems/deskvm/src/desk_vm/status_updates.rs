use dprocess::status_update::StatusUpdate;

use super::DeskVm;

impl DeskVm {
    pub fn flush_status_updates(&self) -> Vec<StatusUpdate> {
        self.status_updates.write().drain(..).collect()
    }
}
