mod cast;

use std::collections::HashMap;
use std::sync::Arc;

use deskc_type::{Effect, Type};
use deskc_type::conclusion::TypeConclusions;
use mir::block::BlockId;
use mir::mir::ControlFlowGraph;
use mir::stmt::{Stmt, Terminator};

use crate::const_stmt;

use crate::value::{Closure, FnRef, Value};
use mir::stmt::StmtBind;
use mir::var::VarId;

#[derive(Debug, Clone, PartialEq)]
pub struct EvalCfg {
    pub(crate) cfg: ControlFlowGraph,
    pub(crate) type_conclusion: Arc<TypeConclusions>,
    pub(crate) registers: HashMap<VarId, Value>,
    pub(crate) parameters: HashMap<Type, Value>,
    pub(crate) captured: HashMap<Type, Value>,
    pub(crate) pc_block: BlockId,
    pub(crate) pc_stmt_idx: usize,
    // Before handling apply stmt, save the var to here, and used when returned.
    pub(crate) return_register: Option<VarId>,
    pub(crate) handlers: HashMap<Effect, Handler>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Handler {
    Handler(Closure),
    Continuation(Vec<EvalCfg>),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum InnerOutput {
    Return(Value),
    Perform {
        input: Value,
        effect: Effect,
    },
    RunOther {
        fn_ref: FnRef,
        parameters: HashMap<Type, Value>,
    },
    Running,
}

impl EvalCfg {
    pub(crate) fn eval_next(&mut self) -> InnerOutput {
        let block = &self.cfg.blocks[self.pc_block.0];
        // if reach to terminator
        if block.stmts.len() == self.pc_stmt_idx {
            match &block.terminator {
                Terminator::Return(var) => InnerOutput::Return(
                    self.registers
                        .remove(var)
                        .expect("return value should be exists"),
                ),
                Terminator::Match { var, cases } => {
                    let value = self.load_value(var);
                    if let Value::Variant { ty, value: _ } = value {
                        let case = cases.iter().find(|c| c.ty == *ty).unwrap();
                        self.pc_block = case.next;
                        self.pc_stmt_idx = 0;
                        InnerOutput::Running
                    } else {
                        panic!("should be variant")
                    }
                }
                Terminator::Goto(next) => {
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
                Stmt::Product(values) => Value::Product(
                    values
                        .iter()
                        .map(|var| (self.get_var_ty(var).clone(), self.load_value(var).clone()))
                        .collect(),
                ),
                Stmt::Vector(_) => todo!(),
                Stmt::Map(_) => todo!(),
                Stmt::Fn(mir::stmt::Closure {
                    mir,
                    captured,
                    handlers,
                }) => Value::FnRef(FnRef::Closure(Closure {
                    mir: *mir,
                    captured: captured
                        .iter()
                        .map(|var| (self.get_var_ty(var).clone(), self.load_value(var).clone()))
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
                })),
                Stmt::Perform(var) => {
                    // Save the return register to get result from continuation.
                    self.return_register = Some(*bind_var);
                    let output = self.get_var_ty(bind_var);
                    let effect = Effect {
                        input: self.get_var_ty(var).clone(),
                        output: output.clone(),
                    };
                    // Increment pc before perform is important
                    self.pc_stmt_idx += 1;
                    return InnerOutput::Perform {
                        input: self.load_value(var).clone(),
                        effect,
                    };
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
                Stmt::Parameter => {
                    // this is safe because typeinfer ensures that a parameter must be exist.
                    let ty = self.get_var_ty(bind_var);
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
                Stmt::MatchResult(_) => todo!(),
                Stmt::Cast(var) => {
                    let value = self.load_value(var);
                    let ty = self.get_var_ty(var);
                    let target = self.get_var_ty(bind_var);
                    EvalCfg::cast(self.type_conclusion.clone(), value, ty, target)
                }
            };
            let var = *bind_var;
            self.store_value(var, value);
            self.pc_stmt_idx += 1;
            InnerOutput::Running
        }
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

    pub fn get_var_ty(&self, var: &VarId) -> &Type {
        &self.cfg.vars.get(var).ty
    }
}

#[cfg(test)]
mod tests {
    use mir::{
        block::BasicBlock,
        scope::{Scope, ScopeId},
        stmt::Const,
        var::{Var, Vars},
    };

    use super::*;

    #[test]
    fn literal() {
        let mir = ControlFlowGraph {
            parameter: Default::default(),
            output: Type::Real,
            vars: Vars(vec![Var {
                ty: Type::Real,
                scope: ScopeId(0),
            }]),
            scopes: vec![Scope { super_scope: None }],
            blocks: vec![BasicBlock {
                stmts: vec![StmtBind {
                    var: VarId(0),
                    stmt: Stmt::Const(Const::Int(1)),
                }],
                terminator: Terminator::Return(VarId(0)),
            }],
            captured: vec![],
            links: vec![],
        };

        let mut eval = EvalCfg {
            cfg: mir,
            type_conclusion: Default::default(),
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
}
