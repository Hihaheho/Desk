use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
    time::Duration,
};

use dprocess::{
    dprocess::{DProcess, DProcessId},
    processor::Processor,
    scheduler::Scheduler,
    status::DProcessStatus,
    status_update::StatusUpdate,
    vm_ref::VmRef,
};

#[derive(Debug, Default)]
/// This is a scheduler that is supported officially.
/// This should have the same capability as Erlang VM's one in the future.
pub struct OfficialScheduler {
    run_queue: VecDeque<Arc<DProcess>>,
    process_status: HashMap<DProcessId, DProcessStatus>,
}

impl Scheduler for OfficialScheduler {
    // An implementation for now.
    // This should reference the Erlang VM's scheduler.
    fn reduce(&mut self, vm: VmRef, _processor: &Processor, target_duration: &Duration) {
        let mut next_queue = VecDeque::new();
        let mut running = vec![];
        while let Some(dprocess) = self.run_queue.pop_front() {
            if let Some(status) = self.process_status.get(&dprocess.id) {
                // Push to next queue because process is attached.
                next_queue.push_back(dprocess.clone());
                if *status == DProcessStatus::Running {
                    running.push(dprocess);
                }
            } else {
                // Process is detached, so do not push to next queue.
                continue;
            }
        }
        let divided_duration = *target_duration / running.len() as u32;
        for dprocess in running {
            dprocess.reduce(vm, &divided_duration);
        }
        self.run_queue = next_queue;
    }

    fn attach(&mut self, dprocess: Arc<DProcess>) {
        self.process_status
            .insert(dprocess.id.clone(), dprocess.read_status().clone());
        self.run_queue.push_back(dprocess);
    }

    fn detach(&mut self, process_id: &DProcessId) {
        self.process_status.remove(process_id);
    }

    fn notify_status(&mut self, status_update: &StatusUpdate) {
        self.process_status
            .entry(status_update.dprocess_id.clone())
            .and_modify(|e| *e = status_update.status.clone());
    }
}
