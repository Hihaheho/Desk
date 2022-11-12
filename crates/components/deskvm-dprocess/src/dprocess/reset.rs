use crate::{interpreter_builder::InterpreterBuilder, status::DProcessStatus, vm_ref::VmRef};

use super::DProcess;

impl DProcess {
    /// Replaces the current interpreter with new one.
    ///
    /// Reset is preferred over loop because it's hot-reloading.
    pub fn reset(&self, vm: VmRef, interpreter_builder: Box<dyn InterpreterBuilder>) {
        let (mut interpreter, mut status) = self.lock_interpreter_and_status();
        *interpreter = interpreter_builder.build();
        self.update_status(vm, &mut status, DProcessStatus::Running);
    }
}
