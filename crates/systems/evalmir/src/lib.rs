pub mod const_stmt;
pub mod eval_mir;
pub mod op_stmt;
pub mod stack;
pub mod value;

use std::collections::HashMap;

use mir::{mir::Mir, ty::ConcType, BlockId};
use stack::StackItem;
use value::Value;

use crate::eval_mir::{EvalMir, InnerOutput};

pub fn eval_mirs(mirs: Vec<Mir>) -> EvalMirs {
    let mir = mirs.get(0).cloned().unwrap();
    EvalMirs {
        mirs,
        stack: vec![EvalMir {
            mir,
            stack: vec![StackItem {
                registers: HashMap::new(),
                parameters: HashMap::new(),
                pc_block: BlockId(0),
                pc_stmt_idx: 0,
            }],
            return_value: None,
        }],
    }
}

pub struct EvalMirs {
    mirs: Vec<Mir>,
    stack: Vec<EvalMir>,
}

impl EvalMirs {
    pub fn eval_next(&mut self) -> Output {
        match self.stack.last_mut().unwrap().eval_next() {
            InnerOutput::Return(value) => {
                if self.stack.len() == 1 {
                    Output::Return(value)
                } else {
                    todo!()
                }
            }
            InnerOutput::Perform { input, output } => todo!(),
            InnerOutput::RunAnothor { mir, parameters } => todo!(),
            InnerOutput::Running => Output::Running,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Output {
    Return(Value),
    Perform { input: Value, output: ConcType },
    Running,
}
