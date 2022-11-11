use std::{ops::DerefMut, time::Duration};

use types::{Effect, Type};

use crate::{
    dprocess_info::DProcessInfo,
    effect_handler::{EffectHandler, HaltProcess, SendMessage},
    interpreter::Interpreter,
    interpreter_output::InterpreterOutput,
    status::DProcessStatus,
    timer::Timer,
    value::Value,
    vm_ref::VmRef,
};

use super::DProcess;

impl DProcess {
    /// Execute the interpreter.
    pub fn reduce(&self, vm: VmRef, target_duration: &Duration) -> ProcessOutput {
        // lock both to prevent invalid state.
        let (mut interpreter, mut status) = self.lock_interpreter_and_status();
        match interpreter.reduce(target_duration) {
            Ok(output) => match output {
                InterpreterOutput::Returned(value) => {
                    *status = DProcessStatus::Finished;
                    ProcessOutput::Returned(value)
                }
                InterpreterOutput::Performed { input, effect } => {
                    self.handle_effect(vm, interpreter, status, effect, input)
                }
                InterpreterOutput::Running => ProcessOutput::Running,
            },
            Err(err) => ProcessOutput::Crashed(err),
        }
    }

    pub fn handle_effect(
        &self,
        vm: VmRef,
        mut interpreter: impl DerefMut<Target = Box<dyn Interpreter>>,
        mut status: impl DerefMut<Target = DProcessStatus>,
        effect: Effect,
        input: Value,
    ) -> ProcessOutput {
        // unwrap is safe because Desk plugins must ensure to .
        // clone is cheap.
        let handler = self.read_effect_handlers().0.get(&effect).unwrap().clone();
        match handler {
            EffectHandler::Immediate(handler) => {
                let output = handler.to_output(&input);
                if let Err(err) = interpreter.effect_output(output) {
                    return ProcessOutput::Crashed(err);
                }
                ProcessOutput::Running
            }
            EffectHandler::Delegation(handler) => {
                let manifest = handler.spawn(&input);
                vm.spawn(manifest);
                ProcessOutput::Delegated
            }
            EffectHandler::Spawn(handler) => {
                let output = handler.to_output(&input);
                if let Err(err) = interpreter.effect_output(output) {
                    return ProcessOutput::Crashed(err);
                }
                let manifest = handler.spawn(&input);
                vm.spawn(manifest);
                ProcessOutput::Running
            }
            EffectHandler::Defer => ProcessOutput::Performed { input, effect },
            EffectHandler::SendMessage(handler) => {
                let output = handler.to_output(&input);
                if let Err(err) = interpreter.effect_output(output) {
                    return ProcessOutput::Crashed(err);
                }
                let SendMessage { to, ty, message } = handler.send_message(&input);
                if let Some(to) = vm.read_dprocesses().get(&to) {
                    to.receive_message(ty, message);
                }
                ProcessOutput::Running
            }
            EffectHandler::ReceiveMessage => {
                let message_type = effect.output;
                // lock mailbox after status is safe.
                if let Some(message) = self
                    .lock_mailbox()
                    .get_mut(&message_type)
                    .and_then(|queue| queue.pop_front())
                {
                    if let Err(err) = interpreter.effect_output(message) {
                        return ProcessOutput::Crashed(err);
                    }
                    ProcessOutput::Running
                } else {
                    *status = DProcessStatus::WaitingForMessage(message_type);
                    ProcessOutput::WaitingForMessage
                }
            }
            EffectHandler::FlushMailbox => {
                let message_type = effect.output;
                // lock mailbox after status is safe.
                let messages = self
                    .lock_mailbox()
                    .get_mut(&message_type)
                    .map(|queue| queue.drain(..).collect())
                    .unwrap_or_else(|| vec![]);
                if let Err(err) = interpreter.effect_output(Value::Vector(messages)) {
                    return ProcessOutput::Crashed(err);
                }
                ProcessOutput::Running
            }
            EffectHandler::Subscribe(handler) => {
                let output = handler.to_output(&input);
                if let Err(err) = interpreter.effect_output(output) {
                    return ProcessOutput::Crashed(err);
                }
                let ty = handler.subscribe(&input);
                vm.subscribe(self.id.clone(), ty);
                ProcessOutput::Running
            }
            EffectHandler::Publish => {
                let ty = effect.input;
                if let Err(err) = interpreter.effect_output(Value::Unit) {
                    return ProcessOutput::Crashed(err);
                }
                vm.publish(ty, input);
                ProcessOutput::Running
            }
            EffectHandler::GetKv(handler) => {
                // read lock KV after status is safe.
                let output = handler.to_output(&input, &self.read_kv());
                if let Err(err) = interpreter.effect_output(output) {
                    return ProcessOutput::Crashed(err);
                }
                ProcessOutput::Running
            }
            EffectHandler::UpdateKv(handler) => {
                // lock KV after status is safe.
                let output = handler.update(&input, &mut self.lock_kv());
                if let Err(err) = interpreter.effect_output(output) {
                    return ProcessOutput::Crashed(err);
                }
                ProcessOutput::Running
            }
            EffectHandler::GetFlags(handler) => {
                let dprocess_id = handler.target_dprocess_id(&input);
                let flags = vm.get_flags(&dprocess_id);
                let output = handler.to_output(&input, flags);
                if let Err(err) = interpreter.effect_output(output) {
                    return ProcessOutput::Crashed(err);
                }
                ProcessOutput::Running
            }
            EffectHandler::UpdateFlags(handler) => {
                let dprocess_id = handler.target_dprocess_id(&input);
                let flags = vm.get_mut_flags(&dprocess_id);
                let output = handler.update_flags(&input, flags);
                if let Err(err) = interpreter.effect_output(output) {
                    return ProcessOutput::Crashed(err);
                }
                ProcessOutput::Running
            }
            EffectHandler::AddTimer(handler) => {
                let output = handler.to_output(&input);
                if let Err(err) = interpreter.effect_output(output) {
                    return ProcessOutput::Crashed(err);
                }
                let manifest = handler.add_timer(&input);
                // lock timers after status is safe.
                self.lock_timers()
                    // TODO: remove clone()
                    .insert(manifest.name.clone(), Timer::new(manifest));
                ProcessOutput::Running
            }
            EffectHandler::RemoveTimer(handler) => {
                let output = handler.to_output(&input);
                if let Err(err) = interpreter.effect_output(output) {
                    return ProcessOutput::Crashed(err);
                }
                let name = handler.remove_timer(&input);
                // lock timers after status is safe.
                self.lock_timers().remove(&name);
                ProcessOutput::Running
            }
            EffectHandler::Monitor(handler) => {
                let output = handler.to_output(&input);
                if let Err(err) = interpreter.effect_output(output) {
                    return ProcessOutput::Crashed(err);
                }
                let target = handler.monitor(&input);
                vm.monitor(self.id.clone(), target);
                ProcessOutput::Running
            }
            EffectHandler::Demonitor(handler) => {
                let output = handler.to_output(&input);
                if let Err(err) = interpreter.effect_output(output) {
                    return ProcessOutput::Crashed(err);
                }
                let target = handler.demonitor(&input);
                vm.demonitor(self.id.clone(), target);
                ProcessOutput::Running
            }
            EffectHandler::ProcessInfo(handler) => {
                // Unlock is required because handler may need read locks of them.
                drop(interpreter);
                drop(status);
                let info = DProcessInfo::new(&self);
                let output = handler.to_output(&input, info);
                // lock interpreter here is safe because we have dropped the locks.
                if let Err(err) = self.lock_interpreter().effect_output(output) {
                    return ProcessOutput::Crashed(err);
                }
                ProcessOutput::Running
            }
            EffectHandler::VmInfo(handler) => {
                let output = handler.to_output(&input, &vm);
                if let Err(err) = interpreter.effect_output(output) {
                    return ProcessOutput::Crashed(err);
                }
                ProcessOutput::Running
            }
            EffectHandler::Link(handler) => {
                let output = handler.to_output(&input);
                if let Err(err) = interpreter.effect_output(output) {
                    return ProcessOutput::Crashed(err);
                }
                let (id1, id2) = handler.link(&input);
                vm.link(id1, id2);
                ProcessOutput::Running
            }
            EffectHandler::Unlink(handler) => {
                let output = handler.to_output(&input);
                if let Err(err) = interpreter.effect_output(output) {
                    return ProcessOutput::Crashed(err);
                }
                let (id1, id2) = handler.unlink(&input);
                vm.unlink(id1, id2);
                ProcessOutput::Running
            }
            EffectHandler::Register(handler) => {
                let output = handler.to_output(&input);
                if let Err(err) = interpreter.effect_output(output) {
                    return ProcessOutput::Crashed(err);
                }
                let (name, id) = handler.register(&input);
                vm.register(name, id);
                ProcessOutput::Running
            }
            EffectHandler::Unregister(handler) => {
                let output = handler.to_output(&input);
                if let Err(err) = interpreter.effect_output(output) {
                    return ProcessOutput::Crashed(err);
                }
                let name = handler.unregister(&input);
                vm.unregister(name);
                ProcessOutput::Running
            }
            EffectHandler::Whereis(handler) => {
                let output = handler.to_output(&input, &vm.read_name_registry());
                if let Err(err) = interpreter.effect_output(output) {
                    return ProcessOutput::Crashed(err);
                }
                ProcessOutput::Running
            }
            EffectHandler::Halt(handler) => {
                let output = handler.to_output(&input);
                if let Err(err) = interpreter.effect_output(output) {
                    return ProcessOutput::Crashed(err);
                }
                let HaltProcess { id, ty, reason } = handler.halt(&input);
                vm.halt_dprocess(id, ty, reason);
                ProcessOutput::Running
            }
        }
    }
}

#[derive(Debug)]
pub enum ProcessOutput {
    Running,
    Delegated,
    WaitingForMessage,
    Performed { input: Value, effect: Effect },
    Returned(Value),
    Halted { ty: Type, reason: Value },
    Crashed(anyhow::Error),
}
