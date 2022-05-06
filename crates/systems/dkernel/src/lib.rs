mod log;
mod queries;
mod snapshot;

use log::LogRepository;
use queries::KernelStorage;

pub struct Kernel {
    pub log_repository: Box<dyn LogRepository>,
    pub db: KernelDatabase,
}

#[salsa::database(KernelStorage)]
#[derive(Default)]
pub struct KernelDatabase {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for KernelDatabase {}
