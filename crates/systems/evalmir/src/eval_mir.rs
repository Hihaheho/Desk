use mir::mir::Mir;

use crate::{const_stmt, op_stmt, stack::StackItem};

use crate::value::Value;
use mir::{ty::ConcType, MirId, StmtBind, VarId};

#[derive(Debug, Clone)]
pub struct EvalMir {
    pub mir: Mir,
    pub stack: Vec<StackItem>,
    // If value is returned, then the value should be used immediately.
    pub return_value: Option<Value>,
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
        let block = &self.mir.blocks[self.stack().pc_block.0];
        let is_top_level = self.stack.len() == 1;
        let current_stack = self.stack.last_mut().unwrap();
        let pc_stmt_idx = current_stack.pc_stmt_idx;
        if block.stmts.len() == pc_stmt_idx {
            // Reach to terminator
            match &block.terminator {
                mir::ATerminator::Return(var) => {
                    if is_top_level {
                        InnerOutput::Return(
                            self.stack
                                .last_mut()
                                .unwrap()
                                .registers
                                .remove(&var)
                                .expect("return value shoulbe exists"),
                        )
                    } else {
                        self.return_value = Some(current_stack.load_value(&var).clone());
                        InnerOutput::Running
                    }
                }
                mir::ATerminator::Match { var, cases } => {
                    let value = current_stack.load_value(&var);
                    dbg!(&cases);
                    if let Value::Variant { id, value: _ } = value {
                        dbg!(&id);
                        let case = cases.iter().find(|c| c.ty == *id).unwrap();
                        current_stack.pc_block = case.next;
                        current_stack.pc_stmt_idx = 0;
                        InnerOutput::Running
                    } else {
                        panic!("should be variant")
                    }
                }
                mir::ATerminator::Goto(next) => {
                    current_stack.pc_block = *next;
                    current_stack.pc_stmt_idx = 0;
                    InnerOutput::Running
                }
            }
        } else {
            let StmtBind { var, stmt } = &block.stmts[current_stack.pc_stmt_idx];
            let value = match stmt {
                mir::stmt::Stmt::Const(const_value) => const_stmt::eval(const_value),
                mir::stmt::Stmt::Tuple(values) => Value::Tuple(
                    values
                        .iter()
                        .map(|var| current_stack.load_value(var).clone())
                        .collect(),
                ),
                mir::stmt::Stmt::Array(_) => todo!(),
                mir::stmt::Stmt::Set(_) => todo!(),
                mir::stmt::Stmt::Fn(_) => todo!(),
                mir::stmt::Stmt::Perform(_) => todo!(),
                mir::stmt::Stmt::Apply {
                    function,
                    arguments,
                } => todo!(),
                mir::stmt::Stmt::Op { op, operands } => match current_stack.eval_op(op, operands) {
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
                // TODO remove old one because move
                mir::stmt::Stmt::Move(x) => current_stack.load_value(x).clone(),
                mir::stmt::Stmt::Variant { id, value } => Value::Variant {
                    id: *id,
                    value: Box::new(current_stack.load_value(value).clone()),
                },
                mir::stmt::Stmt::Parameter => todo!(),
                mir::stmt::Stmt::Returned => self.return_value.take().unwrap(),
            };
            current_stack.store_value(*var, value);
            self.stack.last_mut().unwrap().pc_stmt_idx += 1;
            InnerOutput::Running
        }
    }

    pub fn stack(&self) -> &StackItem {
        self.stack.last().unwrap()
    }

    pub fn stack_mut(&mut self) -> &mut StackItem {
        self.stack.last_mut().unwrap()
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
            stack: vec![StackItem {
                pc_block: BlockId(0),
                pc_stmt_idx: 0,
                registers: HashMap::new(),
                parameters: HashMap::new(),
            }],
            return_value: None,
        };

        assert_eq!(eval.eval_next(), InnerOutput::Running);
        assert_eq!(eval.eval_next(), InnerOutput::Return(Value::Int(1)));
    }

    #[test]
    fn builtin() {}
}
