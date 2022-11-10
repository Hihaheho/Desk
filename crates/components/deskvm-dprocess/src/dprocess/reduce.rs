use std::{ops::DerefMut, time::Duration};

use types::{Effect, Type};

use crate::{
    effect_handler::{EffectHandler, SendMessage},
    interpreter::{self, Interpreter},
    interpreter_output::InterpreterOutput,
    status::DProcessStatus,
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
                InterpreterOutput::Running => todo!(),
            },
            Err(err) => ProcessOutput::Crashed(err),
        }
    }

    pub fn handle_effect(
        &self,
        vm: VmRef,
        interpreter: impl DerefMut<Target = Box<dyn Interpreter>>,
        status: impl DerefMut<Target = DProcessStatus>,
        effect: Effect,
        input: Value,
    ) -> ProcessOutput {
        // unwrap is safe because Desk plugins must ensure to .
        // clone is cheap.
        let handler = self.read_effect_handlers().0.get(&effect).unwrap().clone();
        match handler {
            EffectHandler::Immediate(handler) => {
                let output = handler.to_output(&input);
                interpreter.effect_output(output);
                ProcessOutput::Running
            }
            EffectHandler::Delegation(handler) => {
                let manifest = handler.spawn(&input);
                vm.spawn(manifest);
                ProcessOutput::Delegated
            }
            EffectHandler::Spawn(handler) => {
                let output = handler.to_output(&input);
                interpreter.effect_output(output);
                let manifest = handler.spawn(&input);
                vm.spawn(manifest);
                ProcessOutput::Running
            }
            EffectHandler::Defer => ProcessOutput::Performed { input, effect },
            EffectHandler::SendMessage(handler) => {
                let output = handler.to_output(&input);
                interpreter.effect_output(output);
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
                    interpreter.effect_output(message);
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
                interpreter.effect_output(Value::Vector(messages));
                ProcessOutput::Running
            }
            EffectHandler::Subscribe(handler) => {
                let output = handler.to_output(&input);
                interpreter.effect_output(output);
                // ======================================
                ProcessOutput::Running
            }
            EffectHandler::Publish => todo!(),
            EffectHandler::Get => todo!(),
            EffectHandler::Set => todo!(),
            EffectHandler::List => todo!(),
            EffectHandler::Delete => todo!(),
            EffectHandler::GetFlags(handler) => {
                let output = handler.to_output(&input);
                interpreter.effect_output(output);
                ProcessOutput::Running
            }
            EffectHandler::UpdateFlags(handler) => {
                let output = handler.to_output(&input);
                interpreter.effect_output(output);
                ProcessOutput::Running
            }
            EffectHandler::AddTimer(handler) => {
                let output = handler.to_output(&input);
                interpreter.effect_output(output);
                ProcessOutput::Running
            }
            EffectHandler::RemoveTimer(handler) => {
                let output = handler.to_output(&input);
                interpreter.effect_output(output);
                ProcessOutput::Running
            }
            EffectHandler::Monitor(handler) => {
                let output = handler.to_output(&input);
                interpreter.effect_output(output);
                ProcessOutput::Running
            }
            EffectHandler::Demonitor(handler) => {
                let output = handler.to_output(&input);
                interpreter.effect_output(output);
                ProcessOutput::Running
            }
            EffectHandler::ProcessInfo(handler) => {
                let output = handler.to_output(&input);
                interpreter.effect_output(output);
                ProcessOutput::Running
            }
            EffectHandler::VmInfo(handler) => {
                let output = handler.to_output(&input);
                interpreter.effect_output(output);
                ProcessOutput::Running
            }
            EffectHandler::Link(handler) => {
                let output = handler.to_output(&input);
                interpreter.effect_output(output);
                ProcessOutput::Running
            }
            EffectHandler::Unlink(handler) => {
                let output = handler.to_output(&input);
                interpreter.effect_output(output);
                ProcessOutput::Running
            }
            EffectHandler::Register(handler) => todo!(),
            EffectHandler::Unregister(handler) => todo!(),
            EffectHandler::Whereis(handler) => todo!(),
            EffectHandler::Halt(handler) => {
                let output = handler.to_output(&input);
                interpreter.effect_output(output);
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
