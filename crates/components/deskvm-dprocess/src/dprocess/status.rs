use crate::{
    status::{DProcessStatus, LinkExit},
    vm_ref::VmRef,
};

use super::{
    monitors::{DownMessage, DownPayload},
    DProcess,
};

impl DProcess {
    // Don't lock the status in this method to prevent invalid status.
    /// Pass the lock of status
    pub fn update_status(&self, vm: VmRef, locked: &mut DProcessStatus, status: DProcessStatus) {
        *locked = status;
        let notify_to_monitors = |payload: DownPayload| {
            self.read_monitors()
                .iter()
                .filter_map(|id| vm.get_dprocess(id))
                .for_each(|monitor| {
                    monitor.notify_down(DownMessage {
                        from: self.id.clone(),
                        payload: payload.clone(),
                    });
                });
        };
        let notify_to_links = |link_exit: LinkExit| {
            self.read_links()
                .iter()
                .filter_map(|id| vm.get_dprocess(id))
                .for_each(|link| {
                    link.update_status(
                        vm,
                        &mut link.lock_status(),
                        DProcessStatus::HaltedByLink(link_exit.clone()),
                    );
                });
        };
        match locked {
            DProcessStatus::Returned(value) => {
                notify_to_monitors(DownPayload::Returned(value.clone()));
            }
            DProcessStatus::Halted { ty, reason } => {
                notify_to_monitors(DownPayload::Halted {
                    ty: ty.clone(),
                    reason: reason.clone(),
                });
                notify_to_links(LinkExit::Halted {
                    dprocess_id: self.id.clone(),
                    ty: ty.clone(),
                    reason: reason.clone(),
                });
            }
            DProcessStatus::Crashed(err) => {
                notify_to_monitors(DownPayload::Crashed);
                notify_to_links(LinkExit::Crashed {
                    dprocess_id: self.id.clone(),
                    error: err.clone(),
                });
            }
            DProcessStatus::HaltedByLink(link_exit) => {
                notify_to_monitors(DownPayload::LinkExit(link_exit.clone()));
                notify_to_links(link_exit.clone());
            }
            _ => {}
        }
        // Important! notify to VM's migration logic.
        vm.notify_status(self, locked);
    }
}
