use std::{sync::Arc, time::Duration};

use crate::{
    dprocess::{DProcess, DProcessId},
    processor::Processor,
    vm_output::VmOutputs,
};

pub trait Scheduler: std::fmt::Debug {
    /// Execute attached processes.
    ///
    /// A scheduler never fails.
    /// Implementation should not exceed the given duration.
    /// Implementation can return an output earlier even if it remains codes to run.
    fn reduce(&mut self, processor: &Processor, target_duration: &Duration) -> VmOutputs;

    fn attach(&mut self, dprocess_id: DProcessId, dprocess: Arc<DProcess>);
}
