use parking_lot::RwLock;

use crate::{metas::Metas, scheduler::Scheduler};

#[derive(Debug)]
pub struct Processor {
    pub name: String,
    /// Metadatas mainly used by the scheduler and migration logic.
    pub metas: RwLock<Metas>,
}

#[derive(Debug)]
pub struct ProcessorWithScheduler {
    pub processor: Processor,
    pub scheduler: RwLock<Box<dyn Scheduler>>,
}

#[derive(Debug)]
pub struct ProcessorManifest {
    pub name: ProcessorName,
    pub metas: Metas,
    pub scheduler: Box<dyn Scheduler>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProcessorName(pub String);

impl ProcessorWithScheduler {
    pub fn new(manifest: ProcessorManifest) -> Self {
        Self {
            processor: Processor {
                name: manifest.name.0,
                metas: RwLock::new(manifest.metas),
            },
            scheduler: RwLock::new(manifest.scheduler),
        }
    }
}

impl ProcessorManifest {
    pub fn new(name: ProcessorName, scheduler: impl Scheduler + 'static, metas: Metas) -> Self {
        Self {
            name,
            metas,
            scheduler: Box::new(scheduler),
        }
    }
}
