mod add;
mod eq;
mod sub;

use mir::{stmt::Op, var::VarId};

use crate::{eval_cfg::EvalCfg, value::Value};

impl EvalCfg {
    pub fn eval_op(&self, op: &Op, operands: &[VarId]) -> Value {
        match op {
            Op::Add => {
                assert!(operands.len() == 2);
                add::calc(self.load_value(&operands[0]), self.load_value(&operands[1]))
            }
            Op::Sub => {
                assert!(operands.len() == 2);
                sub::calc(self.load_value(&operands[0]), self.load_value(&operands[1]))
            }
            Op::Mul => todo!(),
            Op::Div => todo!(),
            Op::Rem => todo!(),
            Op::Mod => todo!(),
            Op::Pow => todo!(),
            Op::Eq => {
                assert!(operands.len() == 2);
                eq::calc(self.load_value(&operands[0]), self.load_value(&operands[1]))
            }
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
