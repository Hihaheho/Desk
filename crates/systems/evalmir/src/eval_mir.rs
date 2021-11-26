use std::collections::HashMap;

use mir::mir::Mir;
use mir::BlockId;

use crate::{const_stmt, op_stmt};

use crate::value::{FnRef, Value};
use mir::{ty::ConcType, StmtBind, VarId};

#[derive(Debug, Clone)]
pub struct EvalMir {
    pub mir: Mir,
    pub registers: HashMap<VarId, Value>,
    pub parameters: HashMap<ConcType, Value>,
    pub pc_block: BlockId,
    pub pc_stmt_idx: usize,
    // Before handling apply stmt, save the var to here, and used when returned.
    pub return_register: Option<VarId>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InnerOutput {
    Return(Value),
    Perform {
        input: Value,
        output: ConcType,
    },
    RunOther {
        fn_ref: FnRef,
        parameters: HashMap<ConcType, Value>,
    },
    Running,
}

impl EvalMir {
    pub fn eval_next(&mut self) -> InnerOutput {
        let block = &self.mir.blocks[self.pc_block.0];
        // if reach to terminator
        if block.stmts.len() == self.pc_stmt_idx {
            match &block.terminator {
                mir::ATerminator::Return(var) => InnerOutput::Return(
                    (&mut self.registers)
                        .remove(&var)
                        .expect("return value should be exists"),
                ),
                mir::ATerminator::Match { var, cases } => {
                    let value = self.load_value(&var);
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
            let StmtBind { var: bind_var, stmt } = &block.stmts[self.pc_stmt_idx];
            let value = match stmt {
                mir::stmt::Stmt::Const(const_value) => const_stmt::eval(const_value),
                mir::stmt::Stmt::Tuple(values) => Value::Tuple(
                    values
                        .iter()
                        .map(|var| self.load_value(var).clone())
                        .collect(),
                ),
                mir::stmt::Stmt::Array(_) => todo!(),
                mir::stmt::Stmt::Set(_) => todo!(),
                mir::stmt::Stmt::Fn(fn_ref) => {
                    let fn_ref = match fn_ref {
                        mir::FnRef::Amir(mir) => FnRef::Mir(*mir),
                        mir::FnRef::Link(_) => todo!(),
                        mir::FnRef::Clojure(_, _) => todo!(),
                    };
                    Value::FnRef(fn_ref)
                }
                mir::stmt::Stmt::Perform(_) => todo!(),
                mir::stmt::Stmt::Apply {
                    function,
                    arguments,
                } => {
                    if let Value::FnRef(fn_ref) = self.registers.get(function).cloned().unwrap() {
                        let mut parameters = HashMap::new();
                        arguments.iter().for_each(|arg| {
                            let ty = self.mir.vars.get(arg).ty.clone();
                            let value = self.registers.get(arg).cloned().unwrap();
                            parameters.insert(ty, value);
                        });
                        // Save the return register.
                        self.return_register = Some(*bind_var);
                        // Increment pc before return output is important
                        self.pc_stmt_idx += 1;
                        return InnerOutput::RunOther { fn_ref, parameters };
                    } else {
                        panic!("fn_ref");
                    }
                }
                mir::stmt::Stmt::Op { op, operands } => match self.eval_op(op, operands) {
                    op_stmt::OpResult::Return(value) => value,
                    op_stmt::OpResult::Perform(value) => {
                        // Increment pc before return output is important
                        self.pc_stmt_idx += 1;
                        return InnerOutput::Perform {
                            input: value,
                            output: self.var_type(bind_var),
                        };
                    }
                },
                mir::stmt::Stmt::Ref(_) => todo!(),
                mir::stmt::Stmt::RefMut(_) => todo!(),
                mir::stmt::Stmt::Index { tuple, index } => todo!(),
                // TODO remove old one because move
                mir::stmt::Stmt::Move(x) => self.load_value(x).clone(),
                mir::stmt::Stmt::Variant { id, value } => Value::Variant {
                    id: *id,
                    value: Box::new(self.load_value(value).clone()),
                },
                mir::stmt::Stmt::Parameter => {
                    let ty = &self.mir.vars.get(bind_var).ty;
                    self.parameters
                        .get(ty)
                        .expect("parameter must be exist")
                        .clone()
                }
            };
            let var = *bind_var;
            self.store_value(var, value);
            self.pc_stmt_idx += 1;
            InnerOutput::Running
        }
    }

    // After perform, continue with this function.
    pub fn eval_continue(&mut self, output: Value) {
        todo!()
    }

    // After call another mir, continue with this function.
    pub fn return_value(&mut self, ret: Value) {
        let var = self.return_register.take().expect("needs return register");
        self.store_value(var, ret);
    }

    pub fn load_value(&self, var: &VarId) -> &Value {
        self.registers.get(var).unwrap()
    }

    pub fn store_value(&mut self, var: VarId, value: Value) {
        self.registers.insert(var, value);
    }

    fn var_type(&self, id: &VarId) -> ConcType {
        self.mir.vars.get(id).ty.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use mir::{
        mir::{BasicBlock, Var},
        stmt::Stmt,
        ATerminator, BlockId, Const, Scope, ScopeId, StmtBind, Vars,
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
            captured: vec![],
        };

        let mut eval = EvalMir {
            mir,
            pc_block: BlockId(0),
            pc_stmt_idx: 0,
            registers: HashMap::new(),
            parameters: HashMap::new(),
            return_register: None,
        };

        assert_eq!(eval.eval_next(), InnerOutput::Running);
        assert_eq!(eval.eval_next(), InnerOutput::Return(Value::Int(1)));
    }

    #[test]
    fn builtin() {}
}