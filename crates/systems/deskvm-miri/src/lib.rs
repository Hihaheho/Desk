pub mod const_stmt;
pub mod eval_cfg;
pub mod interpreter_builder;
pub mod operators;
pub mod value;

use anyhow::Result;
use interpreter_builder::{MiriBuilder, MiriBuilderCreationError};
use operators::OPERATORS;
use value::{FnRef, Value};

use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
    time::Duration,
};

use dprocess::{interpreter::Interpreter, interpreter_output::InterpreterOutput};
use mir::{
    block::BlockId,
    mir::{ControlFlowGraph, ControlFlowGraphId, Mir},
};
use ty::{conclusion::TypeConclusions, Effect, Type};

use crate::{
    eval_cfg::{EvalCfg, Handler, InnerOutput},
    value::Closure,
};

pub fn try_create_miri_builder(
    mir: Mir,
    parameters: &HashMap<Type, dprocess::value::Value>,
    type_conclusion: Arc<TypeConclusions>,
) -> Result<MiriBuilder, MiriBuilderCreationError> {
    let parameters = mir
        .captured()
        .into_iter()
        .map(|ty| {
            let parameter = parameters
                .get(ty)
                .cloned()
                .map(Into::into)
                .or_else(|| Some(Value::FnRef(FnRef::Operator(*OPERATORS.get(ty)?))))
                .ok_or_else(|| MiriBuilderCreationError::ParameterNotFound(ty.clone()))?;
            Ok((ty.clone(), parameter))
        })
        .collect::<Result<_, _>>()?;
    Ok(MiriBuilder {
        mir,
        parameters,
        type_conclusion,
    })
}

fn eval_mir(
    mirs: Mir,
    parameters: HashMap<Type, Value>,
    type_conclusion: Arc<TypeConclusions>,
) -> EvalMir {
    let cfg = mirs.cfgs.get(mirs.entrypoint.0).cloned().unwrap();
    EvalMir {
        type_conclusion: type_conclusion.clone(),
        cfgs: mirs.cfgs,
        stack: vec![EvalCfg {
            cfg,
            type_conclusion,
            registers: HashMap::new(),
            parameters,
            captured: HashMap::new(),
            pc_block: BlockId(0),
            pc_stmt_idx: 0,
            return_register: None,
            handlers: HashMap::new(),
        }],
    }
}

#[derive(Clone, Debug)]
pub struct EvalMir {
    type_conclusion: Arc<TypeConclusions>,
    cfgs: Vec<ControlFlowGraph>,
    stack: Vec<EvalCfg>,
}

impl EvalMir {
    pub(crate) fn stack(&mut self) -> &mut EvalCfg {
        self.stack.last_mut().unwrap()
    }

    pub fn get_mir(&self, cfg_id: &ControlFlowGraphId) -> &ControlFlowGraph {
        &self.cfgs[cfg_id.0]
    }

    fn handle_perform(
        &mut self,
        effect: Effect,
        input: Value,
    ) -> anyhow::Result<InterpreterOutput> {
        let mut continuation_from_handler = VecDeque::new();
        let handler = loop {
            if let Some(eval_mir) = self.stack.pop() {
                // find handler
                let handler = eval_mir.handlers.get(&effect).cloned();
                // push eval_mir to continuation
                continuation_from_handler.push_front(eval_mir);
                if let Some(handler) = handler {
                    break handler;
                }
            } else {
                // When handler are not found, push back to continuation stack and perform
                self.stack.extend(continuation_from_handler);
                return Ok(InterpreterOutput::Performed {
                    input: input.into(),
                    effect,
                });
            }
        };
        let output = match handler {
            eval_cfg::Handler::Handler(Closure {
                mir,
                mut captured,
                // Really ignorable??
                handlers: _,
            }) => {
                captured.insert(effect.input.clone(), input);
                let eval_mir = EvalCfg {
                    cfg: self.get_mir(&mir).clone(),
                    type_conclusion: self.type_conclusion.clone(),
                    registers: Default::default(),
                    parameters: Default::default(),
                    captured,
                    pc_block: BlockId(0),
                    pc_stmt_idx: 0,
                    return_register: None,
                    handlers: [(
                        Effect {
                            input: effect.output,
                            output: continuation_from_handler[0].cfg.output.clone(),
                        },
                        Handler::Continuation(continuation_from_handler.into()),
                    )]
                    .into_iter()
                    .collect(),
                };
                self.stack.push(eval_mir);
                InterpreterOutput::Running
            }
            eval_cfg::Handler::Continuation(continuation) => {
                self.stack.extend(continuation_from_handler);
                self.stack.extend(continuation);
                // path input to continuation
                self.stack().return_or_continue_with_value(input);
                InterpreterOutput::Running
            }
        };
        Ok(output)
    }
}

impl Interpreter for EvalMir {
    fn reduce(&mut self, _target_duration: &Duration) -> Result<InterpreterOutput> {
        let output = match self.stack().eval_next() {
            InnerOutput::Return(value) => {
                // When top level
                if self.stack.len() == 1 {
                    InterpreterOutput::Returned(value.into())
                } else {
                    self.stack.pop().unwrap();
                    self.stack().return_or_continue_with_value(value);
                    InterpreterOutput::Running
                }
            }
            InnerOutput::Perform { input, effect } => self.handle_perform(effect, input)?,
            InnerOutput::RunOther { fn_ref, parameters } => match fn_ref {
                value::FnRef::Link(_) => todo!(),
                value::FnRef::Closure(Closure {
                    mir,
                    captured,
                    handlers,
                }) => {
                    let eval_mir = EvalCfg {
                        cfg: self.get_mir(&mir).clone(),
                        type_conclusion: self.type_conclusion.clone(),
                        registers: Default::default(),
                        parameters,
                        captured,
                        pc_block: Default::default(),
                        pc_stmt_idx: Default::default(),
                        return_register: None,
                        handlers,
                    };
                    self.stack.push(eval_mir);
                    InterpreterOutput::Running
                }
                value::FnRef::Recursion => {
                    let eval_mir = EvalCfg {
                        cfg: self.stack().cfg.clone(),
                        type_conclusion: self.type_conclusion.clone(),
                        registers: Default::default(),
                        parameters,
                        captured: self.stack().captured.clone(),
                        pc_block: Default::default(),
                        pc_stmt_idx: Default::default(),
                        return_register: None,
                        handlers: self.stack().handlers.clone(),
                    };
                    self.stack.push(eval_mir);
                    InterpreterOutput::Running
                }
                value::FnRef::Operator(op) => match op.call(&parameters.iter().next().unwrap().1) {
                    value::OperatorOutput::Return(value) => {
                        self.stack().return_or_continue_with_value(value);
                        InterpreterOutput::Running
                    }
                    value::OperatorOutput::Perform { effect, input } => {
                        self.handle_perform(effect, input)?
                    }
                },
            },
            InnerOutput::Running => InterpreterOutput::Running,
        };
        Ok(output)
    }

    fn effect_output(&mut self, value: dprocess::value::Value) {
        self.stack().return_or_continue_with_value(value.into());
    }
}
