mod add;
use mir::{Op, VarId};

use crate::{value::Value, EvalMir};

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
                    &self.get_var(&operands[0]),
                    &self.get_var(&operands[1]),
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
