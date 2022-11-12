use crate::{
    status::{DProcessStatus, LinkExit},
    vm_ref::VmRef,
};

use super::{DProcess, DProcessId};

impl DProcess {
    pub fn add_link(&self, vm_ref: VmRef, link: &DProcess) {
        // Lock the status before update the links
        // Lock the status before links is safe
        let mut self_status = self.lock_status();
        let mut link_status = link.lock_status();

        self.lock_links().insert(link.id.clone());
        link.lock_links().insert(self.id.clone());
        use DProcessStatus::*;
        match (&*self_status, &*link_status) {
            (Running, Halted { ty, reason }) => {
                self.update_status(
                    vm_ref,
                    &mut self_status,
                    HaltedByLink(LinkExit::Halted {
                        dprocess_id: link.id.clone(),
                        ty: ty.clone(),
                        reason: reason.clone(),
                    }),
                );
            }
            (Running, Crashed(err)) => {
                self.update_status(
                    vm_ref,
                    &mut self_status,
                    HaltedByLink(LinkExit::Crashed {
                        dprocess_id: link.id.clone(),
                        error: err.clone(),
                    }),
                );
            }
            (Running, HaltedByLink(exit)) => {
                self.update_status(vm_ref, &mut self_status, HaltedByLink(exit.clone()));
            }
            (Halted { ty, reason }, Running) => {
                link.update_status(
                    vm_ref,
                    &mut link_status,
                    HaltedByLink(LinkExit::Halted {
                        dprocess_id: self.id.clone(),
                        ty: ty.clone(),
                        reason: reason.clone(),
                    }),
                );
            }
            (Crashed(err), Running) => {
                link.update_status(
                    vm_ref,
                    &mut link_status,
                    HaltedByLink(LinkExit::Crashed {
                        dprocess_id: self.id.clone(),
                        error: err.clone(),
                    }),
                );
            }
            (HaltedByLink(exit), Running) => {
                link.update_status(vm_ref, &mut link_status, HaltedByLink(exit.clone()));
            }
            _ => {}
        }
    }

    /// If the d-process is not found when adding a link.
    pub fn link_not_found(&self, vm_ref: VmRef, link: DProcessId) {
        self.update_status(
            vm_ref,
            &mut self.lock_status(),
            DProcessStatus::HaltedByLink(LinkExit::NotFound(link)),
        );
    }

    pub fn remove_link(&self, link: &DProcess) {
        self.lock_links().remove(&link.id);
        link.lock_links().remove(&self.id);
    }
}
