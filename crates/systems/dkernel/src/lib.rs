mod log;
mod queries;
mod snapshot;

use std::collections::HashMap;

use dkernel_card::node::Node;
use dkernel_file::File;
use log::LogRepository;
use queries::KernelStorage;
use uuid::Uuid;

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
