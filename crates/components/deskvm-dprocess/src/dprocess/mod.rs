mod id;
mod new;
mod read_locks;
mod receive_message;
mod reduce;
mod write_locks;

pub use id::DProcessId;
use std::collections::{HashMap, VecDeque};

use parking_lot::RwLock;
use types::Type;

use crate::{
    effect_handler::EffectHandlers, flags::DProcessFlags, interpreter::Interpreter, metas::Metas,
    processor_attachment::ProcessorAttachment, status::DProcessStatus, timer::Timer, value::Value,
};

/// A d-process owns a set of resources and can be scheduled on a processor.
///
/// Heavily inspired by the C struct of Erlang process.
#[derive(Debug)]
// RwLock members are not public to prevent deadlocks.
// Don't reorder members to keep the same order as in the lock methods.
pub struct DProcess {
    /// An interpreter.
    interpreter: RwLock<Box<dyn Interpreter>>,
    /// Metadatas mainly used by the scheduler.
    metas: RwLock<Metas>,
    /// Effect handlers for this process.
    effect_handlers: RwLock<EffectHandlers>,
    /// The status of the process.
    status: RwLock<DProcessStatus>,
    /// Received messages.
    mailbox: RwLock<HashMap<Type, VecDeque<Value>>>,
    /// Which processor is this process attached to.
    processor_attachment: RwLock<ProcessorAttachment>,
    /// A key-value store for this process.
    kv: RwLock<HashMap<Type, Value>>,
    /// This process's flags.
    flags: RwLock<DProcessFlags>,
    /// Attached timers with the name of the counter used for the label of the event.
    timers: RwLock<HashMap<String, Timer>>,
}
