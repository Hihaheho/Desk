use std::{collections::HashMap, sync::Arc};

use ty::{Effect, Type};

use crate::{
    dprocess::DProcessId, dprocess_info::DProcessInfo, dprocess_manifest::DProcessManifest,
    flags::DProcessFlags, name_registry::NameRegistry, timer::TimerManifest, value::Value,
    vm_ref::VmRef,
};

#[derive(Debug, Clone, Default)]
/// Effect handlers attached to a d-process.
///
/// Clone is cheap.
pub struct EffectHandlers(pub HashMap<Effect, EffectHandler>);

#[derive(Debug, Clone)]
/// Inspired by Elixir's `Process` and Cizen.

// To keep this simple:
// - They must be complete within DeskVM, because they are here for better performance.
pub enum EffectHandler {
    /// Immediately computes an output but reports the occurrence.
    ///
    /// This is useful for side-effect only effects such as `print a log`.
    Immediate(Arc<dyn ImmediateEffectHandler>),

    /// Immediately computes an output and spawns a process.
    ///
    /// This is useful for asynchronous effects such as `spawn a process`.
    /// Also, this is useful for delegation effects such as `matrix multiplication` with monitor.
    /// Returns spawned process id.
    Spawn(Arc<dyn SpawnEffectHandler>),

    /// Suspends the process and waits for the effect to be handled in outside of the VM.
    ///
    /// This is useful for IO effects such as `read from network` or `wait for user inputs`.
    Defer,

    /// Send a message to another process.
    SendMessage(Arc<dyn SendMessageEffectHandler>),

    /// Receive a message for a type.
    ///
    /// It blocks the process until a message is received.
    /// A type for message is the output type of the effect.
    /// For setting timeout, use the combination of `AddTimer` and `ReceiveMessage` with Sum type.
    /// The output type of the effect is the type of the message.
    ReceiveMessage,

    /// Flush and list received messages for a type.
    ///
    /// The output type of the effect must be `Vec`, and the item is the type of the message.
    FlushMailbox,

    /// Subscribe to a type.
    Subscribe(Arc<dyn SubscribeEffectHandler>),
    /// Dispatch a message.
    Publish,

    /// Get a value from this process's kv.
    ///
    /// One process can only access its own kv because Desk promotes the idea of message passing than shared memory.
    /// The output type of the effect must be `Sum` of the key type and `@not found *`.
    GetKv(Arc<dyn GetKvEffectHandler>),
    /// Update this process's kv.
    UpdateKv(Arc<dyn UpdateKvEffectHandler>),

    /// Get a process flag.
    GetFlags(Arc<dyn GetFlagsEffectHandler>),
    /// Update a process flag.
    UpdateFlags(Arc<dyn UpdateFlagsEffectHandler>),

    /// Add a timer with the name.
    ///
    /// One process can only manage its own timers to avoid unintended behavior.
    AddTimer(Arc<dyn AddTimerEffectHandler>),
    /// Remove a timer with the name.
    RemoveTimer(Arc<dyn RemoveTimerEffectHandler>),

    /// Monitor a process from this process.
    ///
    /// One process can only manage its own monitors to avoid unintended behavior.
    Monitor(Arc<dyn MonitorEffectHandler>),
    /// Demonitor a process from this process.
    Demonitor(Arc<dyn DemonitorEffectHandler>),

    /// Get a process info for a process ID.
    ///
    /// Available of full process info is not sucure but it is useful.
    /// To secure the information, use sandboxing.
    ProcessInfo(Arc<dyn ProcessInfoEffectHandler>),

    /// Get a VM info.
    ///
    /// Available of full VM info is not sucure but it is useful.
    /// To secure the information, use sandboxing.
    VmInfo(Arc<dyn VmInfoEffectHandler>),

    /// Link a process to another process.
    Link(Arc<dyn LinkEffectHandler>),
    /// Unlink a process from another process.
    Unlink(Arc<dyn UnlinkEffectHandler>),

    /// Register a process with the name.
    Register(Arc<dyn RegisterEffectHandler>),
    /// Unregister the name.
    Unregister(Arc<dyn UnregisterEffectHandler>),

    /// Get the process ID of the process with the name.
    Whereis(Arc<dyn WhereisEffectHandler>),

    /// Halt a process with a reason.
    ///
    /// This is useful for marking an effect should not be happened.
    Halt(Arc<dyn HaltEffectHandler>),
}

pub trait ImmediateEffectHandler: std::fmt::Debug {
    fn to_output(&self, input: &Value) -> Value;
}

pub trait SpawnEffectHandler: std::fmt::Debug {
    fn to_output(&self, input: &Value) -> Value;
    fn spawn(&self, input: &Value) -> DProcessManifest;
}

pub trait SendMessageEffectHandler: std::fmt::Debug {
    fn to_output(&self, input: &Value) -> Value;
    fn send_message(&self, input: &Value) -> SendMessage;
}

pub struct SendMessage {
    pub to: DProcessId,
    pub ty: Type,
    pub message: Value,
}

pub trait SubscribeEffectHandler: std::fmt::Debug {
    fn to_output(&self, input: &Value) -> Value;
    fn subscribe(&self, input: &Value) -> Type;
}

pub trait GetKvEffectHandler: std::fmt::Debug {
    fn to_output(&self, input: &Value, kv: &HashMap<Type, Value>) -> Value;
}

pub trait UpdateKvEffectHandler: std::fmt::Debug {
    /// Returns the output.
    fn update(&self, input: &Value, kv: &mut HashMap<Type, Value>) -> Value;
}

pub trait GetFlagsEffectHandler: std::fmt::Debug {
    fn target_dprocess_id(&self, input: &Value) -> DProcessId;
    fn to_output(&self, input: &Value, flags: Option<&DProcessFlags>) -> Value;
}

pub trait UpdateFlagsEffectHandler: std::fmt::Debug {
    fn target_dprocess_id(&self, input: &Value) -> DProcessId;
    /// Returns the output.
    fn update_flags(&self, input: &Value, flags: Option<&mut DProcessFlags>) -> Value;
}

pub trait AddTimerEffectHandler: std::fmt::Debug {
    fn to_output(&self, input: &Value) -> Value;
    fn add_timer(&self, input: &Value) -> TimerManifest;
}

pub trait RemoveTimerEffectHandler: std::fmt::Debug {
    fn to_output(&self, input: &Value) -> Value;
    fn remove_timer(&self, input: &Value) -> String;
}

pub trait MonitorEffectHandler: std::fmt::Debug {
    fn to_output(&self, input: &Value) -> Value;
    fn monitor(&self, input: &Value) -> DProcessId;
}

pub trait DemonitorEffectHandler: std::fmt::Debug {
    fn to_output(&self, input: &Value) -> Value;
    fn demonitor(&self, input: &Value) -> DProcessId;
}

pub trait ProcessInfoEffectHandler: std::fmt::Debug {
    fn to_output(&self, input: &Value, info: DProcessInfo) -> Value;
}

pub trait VmInfoEffectHandler: std::fmt::Debug {
    fn to_output(&self, input: &Value, info: &VmRef) -> Value;
}

pub trait LinkEffectHandler: std::fmt::Debug {
    fn to_output(&self, input: &Value) -> Value;
    fn link(&self, input: &Value) -> (DProcessId, DProcessId);
}

pub trait UnlinkEffectHandler: std::fmt::Debug {
    fn to_output(&self, input: &Value) -> Value;
    fn unlink(&self, input: &Value) -> (DProcessId, DProcessId);
}

pub trait RegisterEffectHandler: std::fmt::Debug {
    fn to_output(&self, input: &Value) -> Value;
    fn register(&self, input: &Value) -> (String, DProcessId);
}

pub trait UnregisterEffectHandler: std::fmt::Debug {
    fn to_output(&self, input: &Value) -> Value;
    fn unregister(&self, input: &Value) -> String;
}

pub trait WhereisEffectHandler: std::fmt::Debug {
    fn to_output(&self, input: &Value, names: &NameRegistry) -> Value;
}

pub trait HaltEffectHandler: std::fmt::Debug {
    fn to_output(&self, input: &Value) -> Value;
    fn halt(&self, input: &Value) -> HaltProcess;
}

pub struct HaltProcess {
    pub id: DProcessId,
    pub ty: Type,
    pub reason: Value,
}
