pub mod const_stmt;
pub mod value;

use std::collections::HashMap;

use mir::{
    mir::{Mir, Var},
    ty::ConcType,
    BlockId, MirId, StmtBind, VarId,
};
use value::Value;

#[derive(Debug, Clone)]
pub struct EvalMir<'a> {
    pub mir: &'a Mir,
    pub pc_block: BlockId,
    pub pc_stmt_idx: usize,
    // parameters are passed to this.
    pub registers: HashMap<VarId, Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Output {
    Return(Value),
    Perform { input: Value, output: ConcType },
    RunAnothor { mir: MirId, parameters: Vec<Value> },
    Running,
}

impl<'a> EvalMir<'a> {
    pub fn eval_next(&mut self) -> Output {
        let block = &self.mir.blocks[self.pc_block.0];
        if self.pc_stmt_idx == block.stmts.len() {
            // Reach to terminator
            match &block.terminator {
                mir::Terminator::Return(var) => Output::Return(
                    self.registers
                        .remove(&var)
                        .expect("return value shoulbe exists"),
                ),
                mir::Terminator::Match { var, cases } => todo!(),
            }
        } else {
            let StmtBind { var, stmt } = &block.stmts[self.pc_stmt_idx];
            let value = match stmt {
                mir::stmt::Stmt::Const(const_value) => const_stmt::eval(const_value),
                mir::stmt::Stmt::Product(_) => todo!(),
                mir::stmt::Stmt::Array(_) => todo!(),
                mir::stmt::Stmt::Set(_) => todo!(),
                mir::stmt::Stmt::Fn(_) => todo!(),
                mir::stmt::Stmt::Perform(_) => todo!(),
                mir::stmt::Stmt::Apply {
                    function,
                    arguments,
                } => todo!(),
                mir::stmt::Stmt::Op { op, operands } => todo!(),
                mir::stmt::Stmt::Ref(_) => todo!(),
                mir::stmt::Stmt::RefMut(_) => todo!(),
            };
            self.registers.insert(*var, value);
            self.pc_stmt_idx += 1;
            Output::Running
        }
    }

    // After perform, continue with this function.
    pub fn eval_continue(&mut self, output: Value) -> Output {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use mir::{
        mir::{BasicBlock, Var},
        stmt::Stmt,
        Const, Scope, ScopeId, StmtBind, Terminator,
    };

    use super::*;

    #[test]
    fn literal() {
        let mir = Mir {
            parameters: vec![],
            output: ConcType::Number,
            vars: vec![Var {
                ty: ConcType::Number,
                scope: ScopeId(0),
            }],
            scopes: vec![Scope { super_scope: None }],
            blocks: vec![BasicBlock {
                stmts: vec![StmtBind {
                    var: VarId(0),
                    stmt: Stmt::Const(Const::Int(1)),
                }],
                terminator: Terminator::Return(VarId(0)),
            }],
            links: vec![],
        };

        let mut eval = EvalMir {
            mir: &mir,
            pc_block: BlockId(0),
            pc_stmt_idx: 0,
            registers: HashMap::new(),
        };

        assert_eq!(eval.eval_next(), Output::Running);
        assert_eq!(eval.eval_next(), Output::Return(Value::Int(1)));
    }

    #[test]
    fn builtin() {}
}
