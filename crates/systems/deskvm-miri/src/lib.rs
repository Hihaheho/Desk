pub mod const_stmt;
pub mod eval_cfg;
pub mod interpreter_builder;
pub mod value;

use anyhow::Result;

use std::{
    collections::{HashMap, VecDeque},
    time::Duration,
};

use dprocess::{
    interpreter::Interpreter,
    interpreter_output::InterpreterOutput,
    value::{Number, Value},
};
use mir::{
    block::BlockId,
    mir::{ControlFlowGraph, ControlFlowGraphId, Mir},
};
use types::Effect;

use crate::{
    eval_cfg::{EvalCfg, Handler, InnerOutput},
    value::Closure,
};

pub fn eval_mir(mirs: Mir) -> EvalMir {
    let cfg = mirs.cfgs.get(mirs.entrypoint.0).cloned().unwrap();
    EvalMir {
        cfgs: mirs.cfgs,
        stack: vec![EvalCfg {
            cfg,
            registers: HashMap::new(),
            parameters: HashMap::new(),
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
}

impl Interpreter for EvalMir {
    fn reduce(&mut self, _target_duration: &Duration) -> Result<InterpreterOutput> {
        let output = match self.stack().eval_next() {
            InnerOutput::Return(value) => {
                // When top level
                if self.stack.len() == 1 {
                    InterpreterOutput::Returned(to_sendable(value))
                } else {
                    self.stack.pop().unwrap();
                    self.stack().return_or_continue_with_value(value);
                    InterpreterOutput::Running
                }
            }
            InnerOutput::Perform { input, effect } => {
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
                            input: to_sendable(input),
                            effect,
                        });
                    }
                };
                match handler {
                    eval_cfg::Handler::Handler(Closure {
                        mir,
                        mut captured,
                        // Really ignorable??
                        handlers: _,
                    }) => {
                        captured.insert(effect.input.clone(), input);
                        let eval_mir = EvalCfg {
                            cfg: self.get_mir(&mir).clone(),
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
                }
            }
            InnerOutput::RunOther { fn_ref, parameters } => match fn_ref {
                value::FnRef::Link(_) => todo!(),
                value::FnRef::Closure(Closure {
                    mir,
                    captured,
                    handlers,
                }) => {
                    let eval_mir = EvalCfg {
                        cfg: self.get_mir(&mir).clone(),
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
            },
            InnerOutput::Running => InterpreterOutput::Running,
        };
        Ok(output)
    }

    fn effect_output(&mut self, value: Value) {
        self.stack()
            .return_or_continue_with_value(from_sendable(value));
    }
}

fn to_sendable(value: crate::value::Value) -> dprocess::value::Value {
    match value {
        value::Value::Unit => dprocess::value::Value::Unit,
        value::Value::String(string) => dprocess::value::Value::String(string),
        value::Value::Int(int) => dprocess::value::Value::Number(Number::Integer(int)),
        value::Value::Float(float) => dprocess::value::Value::Number(Number::Float(float)),
        value::Value::Rational(a, b) => dprocess::value::Value::Number(Number::Rational(a, b)),
        value::Value::Product(values) => dprocess::value::Value::Product(
            values
                .into_iter()
                .map(|(ty, value)| (ty, to_sendable(value)))
                .collect(),
        ),
        value::Value::Variant { ty, value } => dprocess::value::Value::Variant {
            ty,
            value: Box::new(to_sendable(*value)),
        },
        value::Value::Vector(values) => {
            dprocess::value::Value::Vector(values.into_iter().map(to_sendable).collect())
        }
        value::Value::FnRef(_) => panic!(),
        value::Value::TraitObject { ty, value } => dprocess::value::Value::TraitObject {
            ty,
            value: Box::new(to_sendable(*value)),
        },
    }
}

fn from_sendable(value: Value) -> value::Value {
    match value {
        Value::Unit => value::Value::Unit,
        Value::Number(number) => match number {
            Number::Integer(int) => value::Value::Int(int),
            Number::Float(float) => value::Value::Float(float),
            Number::Rational(a, b) => value::Value::Rational(a, b),
        },
        Value::String(string) => value::Value::String(string),
        Value::Product(values) => value::Value::Product(
            values
                .into_iter()
                .map(|(ty, value)| (ty, from_sendable(value)))
                .collect(),
        ),
        Value::Variant { ty, value } => value::Value::Variant {
            ty,
            value: Box::new(from_sendable(*value)),
        },
        Value::Vector(values) => {
            value::Value::Vector(values.into_iter().map(from_sendable).collect())
        }
        Value::TraitObject { ty, value } => value::Value::TraitObject {
            ty,
            value: Box::new(from_sendable(*value)),
        },
    }
}
