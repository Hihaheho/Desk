pub mod const_stmt;
pub mod op_stmt;
pub mod value;
pub mod var_ext;

use std::collections::HashMap;

use mir::{mir::Mir, ty::ConcType, BlockId, MirId, StmtBind, VarId};
use value::Value;

pub fn eval_mirs(mirs: Vec<Mir>) -> EvalMirs {
    let mir = mirs.get(0).cloned().unwrap();
    EvalMirs {
        mirs,
        stack: vec![EvalMir {
            mir,
            pc_block: BlockId(0),
            pc_stmt_idx: 0,
            registers: HashMap::new(),
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

#[derive(Debug, Clone)]
pub struct EvalMir {
    pub mir: Mir,
    pub pc_block: BlockId,
    pub pc_stmt_idx: usize,
    // parameters are passed to this.
    pub registers: HashMap<VarId, Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Output {
    Return(Value),
    Perform { input: Value, output: ConcType },
    Running,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InnerOutput {
    Return(Value),
    Perform { input: Value, output: ConcType },
    RunAnothor { mir: MirId, parameters: Vec<Value> },
    Running,
}

impl EvalMir {
    pub fn eval_next(&mut self) -> InnerOutput {
        let block = &self.mir.blocks[self.pc_block.0];
        if self.pc_stmt_idx == block.stmts.len() {
            // Reach to terminator
            match &block.terminator {
                mir::ATerminator::Return(var) => InnerOutput::Return(
                    self.registers
                        .remove(&var)
                        .expect("return value shoulbe exists"),
                ),
                mir::ATerminator::Match { var, cases } => {
                    let value = self.get_var(var);
                    if let Value::Variant { id, value: _ } = value {
                        let case = cases.iter().find(|c| c.ty == *id).unwrap();
                        self.pc_block = case.next;
                        self.pc_stmt_idx = 0;
                        InnerOutput::Running
                    } else {
                        panic!("should be variant")
                    }
                }
                mir::ATerminator::Goto(next) => {
                    self.pc_block = *next;
                    self.pc_stmt_idx = 0;
                    InnerOutput::Running
                }
            }
        } else {
            let StmtBind { var, stmt } = &block.stmts[self.pc_stmt_idx];
            let value = match stmt {
                mir::stmt::Stmt::Const(const_value) => const_stmt::eval(const_value),
                mir::stmt::Stmt::Tuple(values) => {
                    Value::Tuple(values.iter().map(|var| self.get_var(var).clone()).collect())
                }
                mir::stmt::Stmt::Array(_) => todo!(),
                mir::stmt::Stmt::Set(_) => todo!(),
                mir::stmt::Stmt::Fn(_) => todo!(),
                mir::stmt::Stmt::Perform(_) => todo!(),
                mir::stmt::Stmt::Apply {
                    function,
                    arguments,
                } => todo!(),
                mir::stmt::Stmt::Op { op, operands } => match self.eval_op(op, operands) {
                    op_stmt::OpResult::Return(value) => value,
                    op_stmt::OpResult::Perform(value) => {
                        return InnerOutput::Perform {
                            input: value,
                            output: self.var_type(var),
                        }
                    }
                },
                mir::stmt::Stmt::Ref(_) => todo!(),
                mir::stmt::Stmt::RefMut(_) => todo!(),
                mir::stmt::Stmt::Index { tuple, index } => todo!(),
                mir::stmt::Stmt::Move(_) => todo!(),
                mir::stmt::Stmt::Variant { id, value } => Value::Variant {
                    id: *id,
                    value: Box::new(self.registers.get(value).cloned().unwrap()),
                },
            };
            self.registers.insert(*var, value);
            self.pc_stmt_idx += 1;
            InnerOutput::Running
        }
    }

    // After perform, continue with this function.
    pub fn eval_continue(&mut self, output: Value) -> InnerOutput {
        todo!()
    }

    // After call another mir, continue with this function.
    pub fn eval_return(&mut self, ret: Value) -> InnerOutput {
        todo!()
    }

    fn var_type(&self, id: &VarId) -> ConcType {
        self.mir.vars.get(id).ty.clone()
    }
}

#[cfg(test)]
mod tests {
    use mir::{
        mir::{BasicBlock, Var},
        stmt::Stmt,
        ATerminator, Const, Scope, ScopeId, StmtBind, Vars,
    };

    use super::*;

    #[test]
    fn literal() {
        let mir = Mir {
            parameters: vec![],
            output: ConcType::Number,
            vars: Vars(vec![Var {
                ty: ConcType::Number,
                scope: ScopeId(0),
            }]),
            scopes: vec![Scope { super_scope: None }],
            blocks: vec![BasicBlock {
                stmts: vec![StmtBind {
                    var: VarId(0),
                    stmt: Stmt::Const(Const::Int(1)),
                }],
                terminator: ATerminator::Return(VarId(0)),
            }],
            links: vec![],
        };

        let mut eval = EvalMir {
            mir,
            pc_block: BlockId(0),
            pc_stmt_idx: 0,
            registers: HashMap::new(),
        };

        assert_eq!(eval.eval_next(), InnerOutput::Running);
        assert_eq!(eval.eval_next(), InnerOutput::Return(Value::Int(1)));
    }

    #[test]
    fn builtin() {}
}
