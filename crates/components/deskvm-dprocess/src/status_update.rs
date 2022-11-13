use crate::{dprocess::DProcessId, status::DProcessStatus};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusUpdate {
    pub dprocess_id: DProcessId,
    pub status: DProcessStatus,
}
