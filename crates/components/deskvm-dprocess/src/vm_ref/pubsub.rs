use types::Type;

use crate::{dprocess::DProcessId, value::Value};

use super::VmRef;
impl<'a> VmRef<'a> {
    pub fn subscribe(&self, _dprocess_id: DProcessId, _ty: Type) {
        todo!()
    }

    pub fn publish(&self, _ty: Type, _value: Value) {
        todo!()
    }
}
