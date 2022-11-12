use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::DerefMut,
};

use types::Type;

use crate::{
    flags::DProcessFlags, interpreter::Interpreter, status::DProcessStatus, timer::Timer,
    value::Value,
};

use super::{DProcess, DProcessId};

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

    pub(crate) fn lock_interpreter_status_mailbox(
        &self,
    ) -> (
        impl DerefMut<Target = Box<dyn Interpreter>> + '_,
        impl DerefMut<Target = DProcessStatus> + '_,
        impl DerefMut<Target = HashMap<Type, VecDeque<Value>>> + '_,
    ) {
        // This order must be the same as DProcess's definition to avoid deadlock.
        (
            self.interpreter.write(),
            self.status.write(),
            self.mailbox.write(),
        )
    }

    pub(crate) fn lock_interpreter(&self) -> impl DerefMut<Target = Box<dyn Interpreter>> + '_ {
        self.interpreter.write()
    }

    pub(crate) fn lock_status(&self) -> impl DerefMut<Target = DProcessStatus> + '_ {
        self.status.write()
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

    pub(crate) fn lock_timers(&self) -> impl DerefMut<Target = HashMap<String, Timer>> + '_ {
        self.timers.write()
    }

    pub(crate) fn lock_monitors(&self) -> impl DerefMut<Target = HashSet<DProcessId>> + '_ {
        self.monitors.write()
    }

    pub(crate) fn lock_links(&self) -> impl DerefMut<Target = HashSet<DProcessId>> + '_ {
        self.links.write()
    }
}
