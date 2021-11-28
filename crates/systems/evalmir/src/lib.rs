pub mod const_stmt;
pub mod eval_mir;
pub mod op_stmt;
pub mod value;

use std::collections::{HashMap, VecDeque};

use mir::{
    mir::{Mir, Mirs},
    ty::ConcEffect,
    BlockId, MirId,
};
use value::Value;

use crate::{
    eval_mir::{EvalMir, Handler, InnerOutput},
    value::Closure,
};

pub fn eval_mirs<'a>(mirs: Mirs) -> EvalMirs {
    let mir = mirs.mirs.get(mirs.entrypoint.0).cloned().unwrap();
    EvalMirs {
        mirs: mirs.mirs,
        stack: vec![EvalMir {
            mir,
            registers: HashMap::new(),
            parameters: HashMap::new(),
            pc_block: BlockId(0),
            pc_stmt_idx: 0,
            return_register: None,
            handlers: HashMap::new(),
        }],
        continuations: Vec::new(),
    }
}

#[derive(Clone, Debug)]
pub struct Continuation {
    continuation: Box<EvalMirs>,
    continuation_effect: ConcEffect,
}

#[derive(Clone, Debug)]
pub struct EvalMirs {
    mirs: Vec<Mir>,
    stack: Vec<EvalMir>,
    continuations: Vec<Continuation>,
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
                    self.stack().return_value(value);
                    Output::Running
                }
            }
            InnerOutput::Perform { input, effect } => {
                dbg!(&effect);
                let mut continuation = VecDeque::new();
                let handler = loop {
                    if let Some(eval_mir) = self.stack.pop() {
                        dbg!(&eval_mir.handlers);
                        // find handler
                        let handler = eval_mir.handlers.get(&effect).cloned();
                        // push eval_mir to continuation
                        continuation.push_front(eval_mir);
                        if let Some(handler) = handler {
                            break handler;
                        }
                    } else {
                        // When handler are not found, push back to continuation stack and perform
                        self.stack.extend(continuation);
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
                        captured.insert(dbg!(effect.input.clone()), dbg!(input));
                        let eval_mir = EvalMir {
                            mir: self.get_mir(&mir).clone(),
                            registers: Default::default(),
                            parameters: captured,
                            pc_block: BlockId(0),
                            pc_stmt_idx: 0,
                            return_register: None,
                            handlers: [dbg!((
                                ConcEffect {
                                    input: effect.output,
                                    output: continuation[0].mir.output.clone(),
                                },
                                Handler::Continuation(continuation.into()),
                            ))]
                            .into_iter()
                            .collect(),
                        };
                        self.stack.push(eval_mir);
                        Output::Running
                    }
                    eval_mir::Handler::Continuation(_) => todo!(),
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
                        pc_block: BlockId(0),
                        pc_stmt_idx: 0,
                        return_register: None,
                        handlers,
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

    pub fn get_mir(&self, mir_id: &MirId) -> &Mir {
        &self.mirs[mir_id.0]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Output {
    Return(Value),
    Perform { input: Value, effect: ConcEffect },
    Running,
}
