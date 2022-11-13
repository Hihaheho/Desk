use std::{sync::Arc, time::Duration};

use crate::{
    dprocess::{DProcess, DProcessId},
    processor::Processor,
    status_update::StatusUpdate,
    vm_ref::VmRef,
};

pub trait Scheduler: std::fmt::Debug {
    /// Execute attached processes.
    ///
    /// A scheduler never fails.
    /// Implementation should not exceed the given duration.
    /// Implementation can return an output earlier even if it remains codes to run.
    fn reduce(&mut self, vm: VmRef, processor: &Processor, target_duration: &Duration);

    fn attach(&mut self, dprocess: Arc<DProcess>);

    fn detach(&mut self, process_id: &DProcessId);

    /// DeskVM calls this method when a status of a process is updated.
    fn notify_status(&mut self, status_update: &StatusUpdate);
}
