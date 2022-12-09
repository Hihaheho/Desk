mod block_proto;
mod mir_proto;
mod scope_proto;

use std::collections::HashMap;

use errors::mirgen::GenMirError;
use hir::{
    expr::{Expr, Handler, Literal},
    meta::WithMeta,
};
use ids::LinkName;
use mir::{
    mir::{ControlFlowGraph, ControlFlowGraphId, Mir},
    stmt::{Closure, Const, MapElem, MatchCase, Stmt, Terminator},
    var::VarId,
};
use mir_proto::MirProto;
use ty::{conclusion::TypeConclusions, Effect, Type};

pub fn gen_mir(expr: &WithMeta<Expr>, conclusion: &TypeConclusions) -> Result<Mir, GenMirError> {
    let mut gen = MirGen::new(conclusion);
    gen.gen_mir(expr).map(|entrypoint_mir_id| Mir {
        entrypoint: entrypoint_mir_id,
        cfgs: gen.mirs,
    })
}

pub struct MirGen<'a> {
    mirs: Vec<ControlFlowGraph>,
    protos: Vec<MirProto>,
    conclusion: &'a TypeConclusions,
}

impl<'a> MirGen<'a> {
    pub fn new(conclusion: &'a TypeConclusions) -> Self {
        Self {
            mirs: vec![],
            protos: vec![MirProto::default()],
            conclusion,
        }
    }
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

impl MirGen<'_> {
    pub fn mir_proto(&mut self) -> &mut MirProto {
        mir_proto!(self)
    }
    pub fn gen_mir(&mut self, hir: &WithMeta<Expr>) -> Result<ControlFlowGraphId, GenMirError> {
        self.begin_mir();
        let var = self.gen_stmt(hir)?;
        let ty = self.get_type(hir)?;
        Ok(self.end_mir(var, ty.clone()))
    }
    pub fn gen_stmt(&mut self, hir: &WithMeta<Expr>) -> Result<VarId, GenMirError> {
        let ty = self.get_type(hir)?.clone();
        self.gen_stmt_ty(hir, &ty)
    }
    pub fn gen_stmt_ty(
        &mut self,
        hir: &WithMeta<Expr>,
        stmt_ty: &Type,
    ) -> Result<VarId, GenMirError> {
        let var_id = match &hir.value {
            Expr::Literal(literal) => {
                let const_value = match literal {
                    Literal::Integer(int) => Stmt::Const(Const::Int(*int)),
                    Literal::String(string) => Stmt::Const(Const::String(string.clone())),
                    Literal::Real(a) => Stmt::Const(Const::Real(*a)),
                    Literal::Rational(a, b) => Stmt::Const(Const::Rational(*a, *b)),
                };
                self.mir_proto().bind_stmt(stmt_ty.clone(), const_value)
            }
            Expr::Vector(values) => {
                let values = values
                    .iter()
                    .map(|value| self.gen_stmt(value))
                    .collect::<Result<Vec<_>, _>>()?;
                self.mir_proto()
                    .bind_stmt(stmt_ty.clone(), Stmt::Vector(values))
            }
            Expr::Map(values) => {
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
            Expr::Do { stmt, expr } => {
                self.gen_stmt(stmt)?;
                self.gen_stmt(expr)?
            }
            Expr::Let { definition, expr } => {
                self.mir_proto().begin_scope();

                // gen definition
                let def_var = if let Expr::Function { parameter: _, body } = &definition.value {
                    let definition_ty = self.get_type(definition)?.clone();
                    let Type::Function(function) = definition_ty.clone() else {
                        return Err(GenMirError::FunctionInferredAsNonFunction {
                            for_expr: definition.meta.clone(),
                        });
                    };
                    // prepare recursion
                    let recursion_var = self
                        .mir_proto()
                        .bind_stmt(definition_ty.clone(), Stmt::Recursion);
                    self.mir_proto().create_named_var(recursion_var);
                    // gen definition
                    let function = self.gen_function(&function.parameter, &body)?;
                    let fn_ref = self.to_closure(function, Default::default());
                    // finish recursion
                    self.mir_proto().bind_stmt(definition_ty, Stmt::Fn(fn_ref))
                } else {
                    // gen definition
                    self.gen_stmt(definition)?
                };

                // make it named
                self.mir_proto().create_named_var(def_var);
                // gen body
                let var = self.gen_stmt(expr)?;

                self.mir_proto().end_scope_then_return(var)
            }
            Expr::Perform { input, output } | Expr::Continue { input, output } => {
                let output = self.get_type(output)?.clone();
                let var = self.gen_stmt(input)?;
                let output_var = self.mir_proto().bind_stmt(output, Stmt::Perform(var));
                self.mir_proto()
                    .bind_stmt(stmt_ty.clone(), Stmt::Cast(output_var))
            }
            Expr::Handle { handlers, expr } => {
                // handlers mir
                let handlers = handlers
                    .iter()
                    .map(|Handler { effect, handler }| {
                        let effect = Effect {
                            input: self.get_type(&effect.input)?.clone(),
                            output: self.get_type(&effect.output)?.clone(),
                        };
                        self.begin_mir();
                        self.set_parameter(effect.input.clone());
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
            Expr::Apply {
                function: function_ty,
                link_name,
                arguments,
            } => {
                let function_ty = self.get_type(function_ty)?.clone();
                let function = if link_name != &LinkName::None {
                    self.mir_proto()
                        .bind_link(function_ty.clone(), link_name.clone())
                } else {
                    self.mir_proto().find_var(&function_ty)
                };
                if arguments.is_empty() {
                    function
                } else {
                    let mut parameters = function_ty.parameters();
                    let arguments = arguments
                        .iter()
                        .map(|arg| {
                            let Some(parameter) = parameters.next() else {
                                return Err(GenMirError::InvalidFunctionCall { expr: hir.meta.clone(), ty: function_ty.clone(), arguments: arguments.into_iter().map(|arg| arg.meta.clone()).collect() });
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
            Expr::Product(values) => {
                let values = values
                    .iter()
                    .map(|value| self.gen_stmt(value))
                    .collect::<Result<Vec<_>, _>>()?;
                self.mir_proto()
                    .bind_stmt(stmt_ty.clone(), Stmt::Product(values))
            }
            Expr::Function { parameter, body } => {
                let parameter = self.get_type(parameter)?.clone();
                let function = self.gen_function(&parameter, body)?;
                let fn_ref = self.to_closure(function, Default::default());
                self.mir_proto()
                    .bind_stmt(stmt_ty.clone(), Stmt::Fn(fn_ref))
            }
            Expr::Match { of, cases } => {
                // gen input
                let sum_type = Type::sum(
                    cases
                        .iter()
                        .map(|c| Ok(self.get_type(&c.ty)?.clone()))
                        .collect::<Result<_, _>>()?,
                );
                let input = self.gen_stmt(of)?;
                let input = self.mir_proto().bind_stmt(sum_type, Stmt::Cast(input));

                // begin and defer the goal block
                let goal_block_id = self.mir_proto().begin_block();
                self.mir_proto().defer_block();

                let match_result_var = self.mir_proto().create_var(stmt_ty.clone());
                let cases: Vec<_> = cases
                    .iter()
                    .map(|hir::expr::MatchCase { ty, expr }| {
                        // gen the case block
                        let case_block_id = self.mir_proto().begin_block();
                        let match_case_result = self.gen_stmt(expr)?;
                        self.mir_proto()
                            .bind_to(match_result_var, Stmt::Cast(match_case_result));
                        // close the last block with goto goal
                        self.mir_proto().end_block(Terminator::Goto(goal_block_id));
                        Ok(MatchCase {
                            ty: self.get_type(ty)?.clone(),
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
            Expr::Label { label: _, item } | Expr::Brand { brand: _, item } => {
                // TODO: simplify this.
                if let Expr::Apply { .. } = item.value {
                    // Reference needs a correct type.
                    let var = self.gen_stmt(item)?;
                    self.mir_proto().bind_stmt(stmt_ty.clone(), Stmt::Cast(var))
                } else {
                    self.gen_stmt_ty(item, stmt_ty)?
                }
            }
            Expr::Typed { ty, item } => {
                let var = self.gen_stmt(item)?;
                let ty = self.get_type(ty)?.clone();
                self.mir_proto().bind_stmt(ty, Stmt::Cast(var))
            }
        };
        Ok(var_id)
    }

    fn gen_function(
        &mut self,
        parameter: &Type,
        body: &WithMeta<Expr>,
    ) -> Result<ControlFlowGraphId, GenMirError> {
        // Begin new mir
        self.begin_mir();

        // make the parameter named
        self.set_parameter(parameter.clone());

        let var = self.gen_stmt(body)?;

        // Out of function
        let ty = self.get_type(body)?.clone();
        Ok(self.end_mir(var, ty))
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

    fn set_parameter(&mut self, parameter: Type) {
        let param_var = self.mir_proto().create_var(parameter.clone());
        self.mir_proto().bind_to(param_var, Stmt::Parameter);
        self.mir_proto().create_named_var(param_var);
        self.mir_proto().set_parameter(parameter);
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

    fn get_type<T: std::fmt::Debug>(&self, hir: &WithMeta<T>) -> Result<&Type, GenMirError> {
        Ok(self
            .conclusion
            .get_type(&hir.meta.id)
            .ok_or_else(|| GenMirError::TypeNotFound {
                for_expr: dbg!(&hir).meta.clone(),
            })?)
    }
}

#[cfg(test)]
mod tests {
    use hir::meta::Meta;
    use mir::{
        scope::{Scope, ScopeId},
        stmt::{StmtBind, Terminator},
        var::{Var, Vars},
    };

    use super::*;

    #[test]
    fn simple() {
        let hir = WithMeta {
            meta: Meta::default(),
            value: Expr::Literal(Literal::Integer(1)),
        };
        let conclusion = TypeConclusions {
            types: [(hir.meta.id.clone(), Type::Integer)].into_iter().collect(),
            cast_strategies: Default::default(),
        };
        let mut gen = MirGen::new(&conclusion);
        gen.gen_mir(&hir).unwrap();

        assert_eq!(gen.mirs.len(), 1);
        assert_eq!(gen.mirs[0].parameter, None);
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
