use std::sync::Arc;

use crate::processor::{ProcessorName, ProcessorWithScheduler};

use super::VmRef;

impl<'a> VmRef<'a> {
    pub fn get_processor(
        &self,
        processor_name: &ProcessorName,
    ) -> Option<Arc<ProcessorWithScheduler>> {
        self.read_processors().get(processor_name).cloned()
    }
}
