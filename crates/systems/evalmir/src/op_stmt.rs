mod add;
use mir::{Op, VarId};

use crate::{eval_mir::EvalMir, value::Value};

pub enum OpResult {
    Return(Value),
    Perform(Value),
}

impl EvalMir {
    pub fn eval_op(&self, op: &Op, operands: &Vec<VarId>) -> OpResult {
        match op {
            Op::Add => {
                assert!(operands.len() == 2);
                OpResult::Return(add::calc(
                    &self.load_value(&operands[0]),
                    &self.load_value(&operands[1]),
                ))
            }

            Op::Sub => todo!(),
            Op::Mul => todo!(),
            Op::Div => todo!(),
            Op::Rem => todo!(),
            Op::Mod => todo!(),
            Op::Pow => todo!(),
            Op::Eq => todo!(),
            Op::Neq => todo!(),
            Op::Lt => todo!(),
            Op::Le => todo!(),
            Op::Gt => todo!(),
            Op::Ge => todo!(),
            Op::Not => todo!(),
            Op::Neg => todo!(),
            Op::Pos => todo!(),
            Op::Shl => todo!(),
            Op::Shr => todo!(),
            Op::BitAnd => todo!(),
            Op::BitOr => todo!(),
            Op::BitXor => todo!(),
            Op::BitNot => todo!(),
        }
    }
}
