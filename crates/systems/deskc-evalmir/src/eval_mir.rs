use std::collections::HashMap;

use mir::mir::ControlFlowGraph;
use mir::stmt::Stmt;
use mir::ty::ConcEffect;
use mir::BlockId;

use crate::const_stmt;

use crate::value::{Closure, FnRef, Value};
use mir::{ty::ConcType, StmtBind, VarId};

#[cfg_attr(feature = "withserde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct EvalMir {
    pub mir: ControlFlowGraph,
    pub registers: HashMap<VarId, Value>,
    pub parameters: HashMap<ConcType, Value>,
    pub captured: HashMap<ConcType, Value>,
    pub pc_block: BlockId,
    pub pc_stmt_idx: usize,
    // Before handling apply stmt, save the var to here, and used when returned.
    pub return_register: Option<VarId>,
    pub handlers: HashMap<ConcEffect, Handler>,
}

#[cfg_attr(feature = "withserde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum Handler {
    Handler(Closure),
    Continuation(Vec<EvalMir>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum InnerOutput {
    Return(Value),
    Perform {
        input: Value,
        effect: ConcEffect,
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
                    self.registers
                        .remove(var)
                        .expect("return value should be exists"),
                ),
                mir::ATerminator::Match { var, cases } => {
                    let value = self.load_value(var);
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
            let StmtBind {
                var: bind_var,
                stmt,
            } = &block.stmts[self.pc_stmt_idx];
            let value = match stmt {
                Stmt::Const(const_value) => const_stmt::eval(const_value),
                Stmt::Tuple(values) => Value::Tuple(
                    values
                        .iter()
                        .map(|var| self.load_value(var).clone())
                        .collect(),
                ),
                Stmt::Array(_) => todo!(),
                Stmt::Set(_) => todo!(),
                Stmt::Fn(fn_ref) => {
                    let fn_ref = match fn_ref {
                        mir::stmt::FnRef::Link(_) => todo!(),
                        mir::stmt::FnRef::Clojure {
                            amir,
                            captured,
                            handlers,
                        } => FnRef::Closure(Closure {
                            mir: *amir,
                            captured: captured
                                .iter()
                                .map(|var| {
                                    (self.get_var_ty(var).clone(), self.load_value(var).clone())
                                })
                                .collect(),
                            handlers: handlers
                                .iter()
                                .map(|(effect, handler)| {
                                    (
                                        effect.clone(),
                                        if let Value::FnRef(FnRef::Closure(closure)) =
                                            self.load_value(handler).clone()
                                        {
                                            Handler::Handler(closure)
                                        } else {
                                            panic!("handler must be FnRef::Closure")
                                        },
                                    )
                                })
                                .collect(),
                        }),
                    };
                    Value::FnRef(fn_ref)
                }
                Stmt::Perform(var) => {
                    // Save the return register to get result from continuation.
                    self.return_register = Some(*bind_var);
                    if let ConcType::Effectful {
                        ty: output,
                        effects: _,
                    } = self.get_var_ty(bind_var)
                    {
                        let effect = ConcEffect {
                            input: self.get_var_ty(var).clone(),
                            output: *output.clone(),
                        };
                        // Increment pc before perform is important
                        self.pc_stmt_idx += 1;
                        return InnerOutput::Perform {
                            input: self.load_value(var).clone(),
                            effect,
                        };
                    } else {
                        panic!("type should be effectful")
                    }
                }
                Stmt::Apply {
                    function,
                    arguments,
                } => {
                    if let Value::FnRef(fn_ref) = self.registers.get(function).cloned().unwrap() {
                        let mut parameters = HashMap::new();
                        arguments.iter().for_each(|arg| {
                            let ty = self.get_var_ty(arg).clone();
                            let value = self.load_value(arg).clone();
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
                Stmt::Op { op, operands } => self.eval_op(op, operands),
                Stmt::Ref(_) => todo!(),
                Stmt::RefMut(_) => todo!(),
                Stmt::Index { tuple: _, index: _ } => todo!(),
                // TODO remove old one because move
                Stmt::Move(x) => self.load_value(x).clone(),
                Stmt::Variant { id, value } => Value::Variant {
                    id: *id,
                    value: Box::new(self.load_value(value).clone()),
                },
                Stmt::Parameter => {
                    let ty = &self.mir.vars.get(bind_var).ty;
                    self.parameters
                        .get(ty)
                        .or_else(|| self.captured.get(ty))
                        .unwrap_or_else(|| {
                            panic!("parameter must be exist {:?} in {:?}", ty, self.parameters)
                        })
                        .clone()
                }
                Stmt::Recursion => Value::FnRef(FnRef::Recursion),
                Stmt::Link(_link) => {
                    todo!()
                }
            };
            let var = *bind_var;
            self.store_value(var, value);
            self.pc_stmt_idx += 1;
            InnerOutput::Running
        }
    }

    // After perform, continue with this function.
    pub fn eval_continue(&mut self, _output: Value) {
        todo!()
    }

    // After call another mir, continue with this function.
    pub fn return_or_continue_with_value(&mut self, ret: Value) {
        let var = self.return_register.take().expect("needs return register");
        self.store_value(var, ret);
    }

    pub fn load_value(&self, var: &VarId) -> &Value {
        self.registers.get(var).unwrap()
    }

    pub fn store_value(&mut self, var: VarId, value: Value) {
        self.registers.insert(var, value);
    }

    pub fn get_var_ty(&self, var: &VarId) -> &ConcType {
        &self.mir.vars.get(var).ty
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
        let mir = ControlFlowGraph {
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
            captured: vec![],
            links: vec![],
        };

        let mut eval = EvalMir {
            mir,
            pc_block: BlockId(0),
            pc_stmt_idx: 0,
            registers: HashMap::new(),
            parameters: HashMap::new(),
            captured: HashMap::new(),
            return_register: None,
            handlers: HashMap::new(),
        };

        assert_eq!(eval.eval_next(), InnerOutput::Running);
        assert_eq!(eval.eval_next(), InnerOutput::Return(Value::Int(1)));
    }

    #[test]
    fn builtin() {}
}
