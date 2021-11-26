pub mod const_stmt;
pub mod eval_mir;
pub mod op_stmt;
pub mod value;

use std::collections::HashMap;

use mir::{
    mir::{Mir, Mirs},
    ty::ConcType,
    BlockId, MirId,
};
use value::Value;

use crate::eval_mir::{EvalMir, InnerOutput};

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
        }],
    }
}

pub struct EvalMirs {
    mirs: Vec<Mir>,
    stack: Vec<EvalMir>,
}

impl EvalMirs {
    pub fn eval_next(&mut self) -> Output {
        match self.stack().eval_next() {
            InnerOutput::Return(value) => {
                println!("------------------");
                println!("{}", self.stack.len());
                // When top level
                if self.stack.len() == 1 {
                    Output::Return(value)
                } else {
                    self.stack.pop().unwrap();
                    self.stack().return_value(value);
                    Output::Running
                }
            }
            InnerOutput::Perform { input, output } => todo!(),
            InnerOutput::RunOther { fn_ref, parameters } => match fn_ref {
                value::FnRef::Mir(mir_id) => {
                    let eval_mir = EvalMir {
                        mir: self.get_mir(&mir_id).clone(),
                        registers: Default::default(),
                        parameters,
                        pc_block: BlockId(0),
                        pc_stmt_idx: 0,
                        return_register: None,
                    };
                    self.stack.push(eval_mir);
                    Output::Running
                }
                value::FnRef::Link(_) => todo!(),
                value::FnRef::Closure { mir, captured } => todo!(),
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
    Perform { input: Value, output: ConcType },
    Running,
}
