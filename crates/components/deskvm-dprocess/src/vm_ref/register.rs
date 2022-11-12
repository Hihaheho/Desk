use crate::dprocess::DProcessId;

use super::VmRef;

impl<'a> VmRef<'a> {
    pub fn register(&self, _name: String, _dprocess_id: DProcessId) {
        todo!()
    }

    pub fn unregister(&self, _name: String) {
        todo!()
    }

    pub fn whereis(&self, _name: String) -> Option<DProcessId> {
        todo!()
    }
}
