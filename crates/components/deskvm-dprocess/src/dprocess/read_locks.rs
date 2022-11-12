use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::Deref,
};

use types::Type;

use crate::{
    effect_handler::EffectHandlers, flags::DProcessFlags, interpreter::Interpreter, metas::Metas,
    processor_attachment::ProcessorAttachment, status::DProcessStatus, timer::Timer, value::Value,
};

use super::{DProcess, DProcessId};

/// Read locks.
impl DProcess {
    /// Locks the interpreter for reading.
    pub fn read_interpreter(&self) -> impl Deref<Target = Box<dyn Interpreter>> + '_ {
        self.interpreter.read()
    }

    /// Locks the metas for reading.
    pub fn read_metas(&self) -> impl Deref<Target = Metas> + '_ {
        self.metas.read()
    }

    /// Locks the effect handlers for reading.
    pub fn read_effect_handlers(&self) -> impl Deref<Target = EffectHandlers> + '_ {
        self.effect_handlers.read()
    }

    /// Locks the status for reading.
    pub fn read_status(&self) -> impl Deref<Target = DProcessStatus> + '_ {
        self.status.read()
    }

    /// Locks the mailbox for reading.
    pub fn read_mailbox(&self) -> impl Deref<Target = HashMap<Type, VecDeque<Value>>> + '_ {
        self.mailbox.read()
    }

    /// Locks the processor attachment for reading.
    pub fn read_processor_attachment(&self) -> impl Deref<Target = ProcessorAttachment> + '_ {
        self.processor_attachment.read()
    }

    /// Locks the kv for reading.
    pub fn read_kv(&self) -> impl Deref<Target = HashMap<Type, Value>> + '_ {
        self.kv.read()
    }

    /// Locks the flags for reading.
    pub fn read_flags(&self) -> impl Deref<Target = DProcessFlags> + '_ {
        self.flags.read()
    }

    /// Locks the timers for reading.
    pub fn read_timers(&self) -> impl Deref<Target = HashMap<String, Timer>> + '_ {
        self.timers.read()
    }

    /// Locks the monitors for reading.
    pub fn read_monitors(&self) -> impl Deref<Target = HashSet<DProcessId>> + '_ {
        self.monitors.read()
    }

    /// Locks the links for reading.
    pub fn read_links(&self) -> impl Deref<Target = HashSet<DProcessId>> + '_ {
        self.links.read()
    }
}
