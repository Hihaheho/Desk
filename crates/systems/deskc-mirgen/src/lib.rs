mod block_proto;
mod mir_proto;
mod scope_proto;

use std::collections::HashMap;

use mir::{
    mir::{ControlFlowGraph, ControlFlowGraphId, Mir},
    stmt::{Closure, Const, MapElem, MatchCase, Stmt, Terminator},
    var::VarId,
};
use mir_proto::MirProto;
use thir::{Handler, LinkName, TypedHir};
use thiserror::Error;
use types::{Effect, Type};

pub fn gen_mir(thir: &TypedHir) -> Result<Mir, GenMirError> {
    let mut gen = MirGen::default();
    gen.gen_mir(thir).map(|entrypoint_mir_id| Mir {
        entrypoint: entrypoint_mir_id,
        cfgs: gen.mirs,
    })
}

pub struct MirGen {
    mirs: Vec<ControlFlowGraph>,
    protos: Vec<MirProto>,
}

impl Default for MirGen {
    fn default() -> Self {
        Self {
            mirs: vec![],
            protos: vec![MirProto::default()],
        }
    }
}

#[derive(Debug, Clone, Error)]
pub enum GenMirError {
    #[error("invalid function call {function:?} with {arguments:?}")]
    InvalidFunctionCall {
        function: Type,
        arguments: Vec<TypedHir>,
    },
}

macro_rules! mir_proto {
    ($ctx:expr) => {
        $ctx.protos.last_mut().unwrap()
    };
}

macro_rules! get_mir {
    ($ctx:expr, $id:expr) => {
        &$ctx.mirs[$id.0]
    };
}

impl MirGen {
    pub fn mir_proto(&mut self) -> &mut MirProto {
        mir_proto!(self)
    }
    pub fn gen_mir(&mut self, thir: &TypedHir) -> Result<ControlFlowGraphId, GenMirError> {
        self.begin_mir();
        let var = self.gen_stmt(thir)?;
        Ok(self.end_mir(var, thir.ty.clone()))
    }
    pub fn gen_stmt(&mut self, thir: &TypedHir) -> Result<VarId, GenMirError> {
        let TypedHir {
            id: _,
            ty: stmt_ty,
            expr,
        } = thir;
        let var_id = match expr {
            thir::Expr::Literal(literal) => {
                let const_value = match literal {
                    thir::Literal::Int(int) => Stmt::Const(Const::Int(*int)),
                    thir::Literal::String(string) => Stmt::Const(Const::String(string.clone())),
                    thir::Literal::Real(a) => Stmt::Const(Const::Real(*a)),
                    thir::Literal::Rational(a, b) => Stmt::Const(Const::Rational(*a, *b)),
                };
                self.mir_proto().bind_stmt(stmt_ty.clone(), const_value)
            }
            thir::Expr::Vector(values) => {
                let values = values
                    .iter()
                    .map(|value| self.gen_stmt(value))
                    .collect::<Result<Vec<_>, _>>()?;
                self.mir_proto()
                    .bind_stmt(stmt_ty.clone(), Stmt::Vector(values))
            }
            thir::Expr::Map(values) => {
                let values = values
                    .iter()
                    .map(|elem| {
                        Ok(MapElem {
                            key: self.gen_stmt(&elem.key)?,
                            value: self.gen_stmt(&elem.value)?,
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                self.mir_proto()
                    .bind_stmt(stmt_ty.clone(), Stmt::Map(values))
            }
            thir::Expr::Do { stmt, expr } => {
                self.gen_stmt(stmt)?;
                self.gen_stmt(expr)?
            }
            thir::Expr::Let { definition, body } => {
                self.mir_proto().begin_scope();

                // gen definition
                let def_var = if let thir::Expr::Function { parameter, body } = &definition.expr {
                    // prepare recursion
                    let recursion_var = self
                        .mir_proto()
                        .bind_stmt(definition.ty.clone(), Stmt::Recursion);
                    self.mir_proto().create_named_var(recursion_var);
                    // gen definition
                    let function = self.gen_function(parameter, body)?;
                    let fn_ref = self.to_closure(function, Default::default());
                    // finish recursion
                    self.mir_proto()
                        .bind_stmt(definition.ty.clone(), Stmt::Fn(fn_ref))
                } else {
                    // gen definition
                    self.gen_stmt(definition)?
                };

                // make it named
                self.mir_proto().create_named_var(def_var);
                // gen body
                let var = self.gen_stmt(body)?;

                self.mir_proto().end_scope_then_return(var)
            }
            thir::Expr::Perform { input, output } => {
                let var = self.gen_stmt(input)?;
                let output_var = self
                    .mir_proto()
                    .bind_stmt(output.clone(), Stmt::Perform(var));
                self.mir_proto()
                    .bind_stmt(stmt_ty.clone(), Stmt::Cast(output_var))
            }
            thir::Expr::Handle { handlers, expr } => {
                let handlers = handlers
                    .iter()
                    .map(|Handler { effect, handler }| {
                        self.begin_mir();
                        let handler_end = self.gen_stmt(handler)?;
                        let handler_cfg_id = self.end_mir(handler_end, stmt_ty.clone());
                        let handler_type = self.get_mir(&handler_cfg_id).get_type().clone();
                        // call effectful mir
                        let fn_ref = self.to_closure(handler_cfg_id, Default::default());
                        let handler_var =
                            self.mir_proto().bind_stmt(handler_type, Stmt::Fn(fn_ref));
                        Ok((effect.clone(), handler_var))
                    })
                    .collect::<Result<HashMap<_, _>, _>>()?;
                // handler mir

                // effectful mir
                self.begin_mir();
                let effectful_end = self.gen_stmt(expr)?;
                let effectful_cfg_id = self.end_mir(effectful_end, stmt_ty.clone());
                let effectful_type = self.get_mir(&effectful_cfg_id).get_type().clone();

                let fn_ref = self.to_closure(effectful_cfg_id, handlers);
                let effectful_fun = self.mir_proto().bind_stmt(effectful_type, Stmt::Fn(fn_ref));
                self.mir_proto().bind_stmt(
                    stmt_ty.clone(),
                    Stmt::Apply {
                        function: effectful_fun,
                        arguments: vec![],
                    },
                )
            }
            thir::Expr::Apply {
                function: function_ty,
                link_name,
                arguments,
            } => {
                let function = if link_name != &LinkName::None {
                    self.mir_proto()
                        .bind_link(function_ty.clone(), link_name.clone())
                } else {
                    self.mir_proto().find_var(function_ty)
                };
                if arguments.is_empty() {
                    function
                } else {
                    let mut parameters = function_ty.parameters();
                    let arguments = arguments
                        .iter()
                        .map(|arg| {
                            let Some(parameter) = parameters.next() else {
                                return Err(GenMirError::InvalidFunctionCall { function: function_ty.clone(), arguments: arguments.clone() });
                            };
                            let var = self.gen_stmt(arg)?;
                            Ok(self.mir_proto().bind_stmt(parameter.clone(), Stmt::Cast(var)))
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    self.mir_proto().bind_stmt(
                        stmt_ty.clone(),
                        Stmt::Apply {
                            function,
                            arguments,
                        },
                    )
                }
            }
            thir::Expr::Product(values) => {
                let values = values
                    .iter()
                    .map(|value| self.gen_stmt(value))
                    .collect::<Result<Vec<_>, _>>()?;
                self.mir_proto()
                    .bind_stmt(stmt_ty.clone(), Stmt::Product(values))
            }
            thir::Expr::Function { parameter, body } => {
                let function = self.gen_function(parameter, body)?;
                let fn_ref = self.to_closure(function, Default::default());
                self.mir_proto()
                    .bind_stmt(stmt_ty.clone(), Stmt::Fn(fn_ref))
            }
            thir::Expr::Match { input, cases } => {
                // gen input
                let sum_type = Type::sum(cases.iter().map(|c| c.ty.clone()).collect());
                let input = self.gen_stmt(input)?;
                let input = self.mir_proto().bind_stmt(sum_type, Stmt::Cast(input));

                // begin and defer the goal block
                let goal_block_id = self.mir_proto().begin_block();
                self.mir_proto().defer_block();

                let match_result_var = self.mir_proto().create_var(stmt_ty.clone());
                let cases: Vec<_> = cases
                    .iter()
                    .map(|thir::MatchCase { ty, expr }| {
                        // gen the case block
                        let case_block_id = self.mir_proto().begin_block();
                        let match_case_result = self.gen_stmt(expr)?;
                        self.mir_proto()
                            .bind_to(match_result_var, Stmt::Cast(match_case_result));
                        // close the last block with goto goal
                        self.mir_proto().end_block(Terminator::Goto(goal_block_id));
                        Ok(MatchCase {
                            ty: ty.clone(),
                            next: case_block_id,
                        })
                    })
                    .collect::<Result<_, _>>()?;
                self.mir_proto()
                    .end_block(Terminator::Match { var: input, cases });
                // undefer the goal block
                self.mir_proto().pop_deferred_block();
                match_result_var
            }
            thir::Expr::Label {
                label: _,
                item: expr,
            } => {
                // TODO: simplify this.
                if let thir::Expr::Apply { .. } = expr.expr {
                    // Reference needs a correct type.
                    let var = self.gen_stmt(&TypedHir {
                        id: expr.id.clone(),
                        ty: expr.ty.clone(),
                        expr: expr.expr.clone(),
                    })?;
                    self.mir_proto().bind_stmt(stmt_ty.clone(), Stmt::Cast(var))
                } else {
                    self.gen_stmt(&TypedHir {
                        id: expr.id.clone(),
                        ty: stmt_ty.clone(),
                        expr: expr.expr.clone(),
                    })?
                }
            }
        };
        Ok(var_id)
    }

    fn gen_function(
        &mut self,
        parameter: &Type,
        body: &TypedHir,
    ) -> Result<ControlFlowGraphId, GenMirError> {
        // Begin new mir
        self.begin_mir();

        // make the parameter named
        let param_var = self.mir_proto().create_var(parameter.clone());
        self.mir_proto().bind_to(param_var, Stmt::Parameter);
        self.mir_proto().create_named_var(param_var);

        let var = self.gen_stmt(body)?;

        // Out of function
        Ok(self.end_mir(var, body.ty.clone()))
    }

    fn to_closure(
        &mut self,
        cfg_id: ControlFlowGraphId,
        handlers: HashMap<Effect, VarId>,
    ) -> Closure {
        let mir = get_mir!(self, cfg_id);
        let captured = mir
            .captured
            .iter()
            .map(|ty| mir_proto!(self).find_var(ty))
            .collect();

        Closure {
            mir: cfg_id,
            captured,
            handlers,
        }
    }

    fn begin_mir(&mut self) {
        self.protos.push(MirProto::default());
    }

    fn end_mir(&mut self, var: VarId, ty: Type) -> ControlFlowGraphId {
        let proto = self.protos.pop().expect("mir must be started");
        let id = ControlFlowGraphId(self.mirs.len());
        self.mirs.push(proto.into_mir(var, ty));
        id
    }

    fn get_mir(&self, id: &ControlFlowGraphId) -> &ControlFlowGraph {
        get_mir!(self, id)
    }
}

#[cfg(test)]
mod tests {
    use ids::NodeId;
    use mir::{
        scope::{Scope, ScopeId},
        stmt::{StmtBind, Terminator},
        var::{Var, Vars},
    };

    use super::*;

    #[test]
    fn simple() {
        let thir = TypedHir {
            id: NodeId::default(),
            ty: Type::Integer,
            expr: thir::Expr::Literal(thir::Literal::Int(1)),
        };
        let mut gen = MirGen::default();
        gen.gen_mir(&thir).unwrap();

        assert_eq!(gen.mirs.len(), 1);
        assert_eq!(gen.mirs[0].parameters, vec![]);
        assert_eq!(gen.mirs[0].scopes, vec![Scope { super_scope: None }]);
        assert_eq!(
            gen.mirs[0].vars,
            Vars(vec![Var {
                ty: Type::Integer,
                scope: ScopeId(0)
            }])
        );
        assert_eq!(gen.mirs[0].blocks.len(), 1);
        assert_eq!(
            gen.mirs[0].blocks[0].stmts,
            vec![StmtBind {
                var: VarId(0),
                stmt: Stmt::Const(Const::Int(1)),
            }]
        );
        assert_eq!(
            gen.mirs[0].blocks[0].terminator,
            Terminator::Return(VarId(0))
        );
    }
}
