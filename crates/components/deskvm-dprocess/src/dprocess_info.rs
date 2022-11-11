use crate::dprocess::{DProcess, DProcessId};

// This must be private to prevent write locks.
pub struct DProcessInfo<'a> {
    reference: &'a DProcess,
}

impl<'a> DProcessInfo<'a> {
    pub fn new(reference: &'a DProcess) -> Self {
        Self { reference }
    }

    pub fn id(&self) -> &DProcessId {
        &self.reference.id
    }

    // TODO Add read_* methods.
}
