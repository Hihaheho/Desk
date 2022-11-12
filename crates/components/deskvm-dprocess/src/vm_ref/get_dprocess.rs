use std::sync::Arc;

use crate::dprocess::{DProcess, DProcessId};

use super::VmRef;

impl<'a> VmRef<'a> {
    pub fn get_dprocess(&self, dprocess_id: &DProcessId) -> Option<Arc<DProcess>> {
        self.read_dprocesses().get(dprocess_id).cloned()
    }
}
