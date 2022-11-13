use std::time::Duration;

use super::DeskVm;

impl DeskVm {
    // VM never fails.
    /// An API for single-threaded platform such as the Web or realtime application like games.
    pub fn reduce(&mut self, target_duration: &Duration) {
        // This is a single threaded version.
        let divided_duration = *target_duration / self.processors.read().len() as u32;
        for pws in self.processors.read().values() {
            pws.scheduler
                .write()
                .reduce(self.vm_ref(), &pws.processor, &divided_duration)
        }
    }
}
