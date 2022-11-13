use types::Type;

use crate::{status::DProcessStatus, value::Value, vm_ref::VmRef};

use super::DProcess;

impl DProcess {
    pub fn receive_message(&self, vm: VmRef, ty: Type, value: Value) {
        let (mut interpreter, mut status, mut mailbox) = self.lock_interpreter_status_mailbox();
        if let DProcessStatus::WaitingForMessage(waiting_for) = &*status {
            if **waiting_for == ty {
                interpreter.effect_output(value);
                self.update_status(vm, &mut status, DProcessStatus::Running);
                return;
            }
        }
        mailbox.entry(ty).or_default().push_back(value);
    }
}
