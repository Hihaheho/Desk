use parking_lot::RwLock;

use crate::{metas::Metas, scheduler::Scheduler};

#[derive(Debug)]
pub struct Processor {
    /// Metadatas mainly used by the scheduler and migration logic.
    pub metas: RwLock<Metas>,
}

#[derive(Debug)]
pub struct ProcessorWithScheduler {
    pub processor: RwLock<Processor>,
    pub scheduler: RwLock<Box<dyn Scheduler>>,
}
