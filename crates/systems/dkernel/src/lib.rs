mod log;
mod queries;
mod query_result;
mod snapshot;

use log::LogRepository;
use queries::KernelStorage;
use snapshot::Snapshot;

pub struct Kernel {
    pub snapshot: Snapshot,
    pub log_repository: Box<dyn LogRepository>,
    pub db: KernelDatabase,
}

#[salsa::database(KernelStorage)]
#[derive(Default)]
pub struct KernelDatabase {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for KernelDatabase {}
