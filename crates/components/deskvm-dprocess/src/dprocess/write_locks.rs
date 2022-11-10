use std::{
    collections::{HashMap, VecDeque},
    ops::DerefMut,
};

use types::Type;

use crate::{interpreter::Interpreter, status::DProcessStatus, value::Value, flags::DProcessFlags};

use super::DProcess;

// These must be private to prevent deadlocks.
impl DProcess {
    pub(crate) fn lock_interpreter_and_status(
        &self,
    ) -> (
        impl DerefMut<Target = Box<dyn Interpreter>> + '_,
        impl DerefMut<Target = DProcessStatus> + '_,
    ) {
        // This order must be the same as DProcess's definition to avoid deadlock.
        (self.interpreter.write(), self.status.write())
    }

    pub(crate) fn lock_status_and_mailbox(
        &self,
    ) -> (
        impl DerefMut<Target = DProcessStatus> + '_,
        impl DerefMut<Target = HashMap<Type, VecDeque<Value>>> + '_,
    ) {
        // This order must be the same as DProcess's definition to avoid deadlock.
        (self.status.write(), self.mailbox.write())
    }

    pub(crate) fn lock_mailbox(
        &self,
    ) -> impl DerefMut<Target = HashMap<Type, VecDeque<Value>>> + '_ {
        self.mailbox.write()
    }

    pub(crate) fn lock_kv(&self) -> impl DerefMut<Target = HashMap<Type, Value>> + '_ {
        self.kv.write()
    }

    pub(crate) fn lock_flags(&self) -> impl DerefMut<Target = DProcessFlags> + '_ {
        self.flags.write()
    }
}
