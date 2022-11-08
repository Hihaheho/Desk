pub mod const_stmt;
pub mod eval_mir;
pub mod op_stmt;
pub mod value;

use std::collections::{HashMap, VecDeque};

use mir::{
    mir::{ControlFlowGraph, Mir},
    ty::ConcEffect,
    BlockId, MirId,
};
use value::Value;

use crate::{
    eval_mir::{EvalMir, Handler, InnerOutput},
    value::Closure,
};

pub fn eval_mirs(mirs: Mir) -> EvalMirs {
    let mir = mirs.mirs.get(mirs.entrypoint.0).cloned().unwrap();
    EvalMirs {
        mirs: mirs.mirs,
        stack: vec![EvalMir {
            mir,
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
pub struct EvalMirs {
    mirs: Vec<ControlFlowGraph>,
    stack: Vec<EvalMir>,
}

impl EvalMirs {
    pub fn eval_next(&mut self) -> Output {
        match self.stack().eval_next() {
            InnerOutput::Return(value) => {
                // When top level
                if self.stack.len() == 1 {
                    Output::Return(value)
                } else {
                    self.stack.pop().unwrap();
                    self.stack().return_or_continue_with_value(value);
                    Output::Running
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
                        return Output::Perform { input, effect };
                    }
                };
                match handler {
                    eval_mir::Handler::Handler(Closure {
                        mir,
                        mut captured,
                        // Really ignorable??
                        handlers: _,
                    }) => {
                        captured.insert(effect.input.clone(), input);
                        let eval_mir = EvalMir {
                            mir: self.get_mir(&mir).clone(),
                            registers: Default::default(),
                            parameters: Default::default(),
                            captured,
                            pc_block: BlockId(0),
                            pc_stmt_idx: 0,
                            return_register: None,
                            handlers: [(
                                ConcEffect {
                                    input: effect.output,
                                    output: continuation_from_handler[0].mir.output.clone(),
                                },
                                Handler::Continuation(continuation_from_handler.into()),
                            )]
                            .into_iter()
                            .collect(),
                        };
                        self.stack.push(eval_mir);
                        Output::Running
                    }
                    eval_mir::Handler::Continuation(continuation) => {
                        self.stack.extend(continuation_from_handler);
                        self.stack.extend(continuation);
                        // path input to continuation
                        self.stack().return_or_continue_with_value(input);
                        Output::Running
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
                    let eval_mir = EvalMir {
                        mir: self.get_mir(&mir).clone(),
                        registers: Default::default(),
                        parameters,
                        captured,
                        pc_block: Default::default(),
                        pc_stmt_idx: Default::default(),
                        return_register: None,
                        handlers,
                    };
                    self.stack.push(eval_mir);
                    Output::Running
                }
                value::FnRef::Recursion => {
                    let eval_mir = EvalMir {
                        mir: self.stack().mir.clone(),
                        registers: Default::default(),
                        parameters,
                        captured: self.stack().captured.clone(),
                        pc_block: Default::default(),
                        pc_stmt_idx: Default::default(),
                        return_register: None,
                        handlers: self.stack().handlers.clone(),
                    };
                    self.stack.push(eval_mir);
                    Output::Running
                }
            },
            InnerOutput::Running => Output::Running,
        }
    }

    pub fn stack(&mut self) -> &mut EvalMir {
        self.stack.last_mut().unwrap()
    }

    pub fn get_mir(&self, mir_id: &MirId) -> &ControlFlowGraph {
        &self.mirs[mir_id.0]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Output {
    Return(Value),
    Perform { input: Value, effect: ConcEffect },
    Running,
}
