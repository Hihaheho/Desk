mod amir_proto;
mod block_proto;
mod into_op;
mod scope_proto;

use amir::{
    amir::{Amir, AmirId},
    block::BlockId,
    stmt::{AStmt, ATerminator, Const, FnRef, MatchCase},
    var::VarId,
};
use amir_proto::AmirProto;
use thir::TypedHir;
use thiserror::Error;
use types::Type;

pub fn gen_abstract_mir(thir: &TypedHir) -> Result<Vec<Amir>, GenAmirError> {
    let mut gen = AmirGen::default();
    gen.gen_amir(thir).map(|_id| gen.amirs)
}

pub struct AmirGen {
    amirs: Vec<Amir>,
    protos: Vec<AmirProto>,
}

impl Default for AmirGen {
    fn default() -> Self {
        Self {
            amirs: vec![],
            protos: vec![AmirProto::default()],
        }
    }
}

#[derive(Debug, Clone, Error)]
pub enum GenAmirError {
    #[error("reference unknown var {0:?}")]
    ReferencesUnknownVar(Type),
}

impl AmirGen {
    pub fn amir(&mut self) -> &mut AmirProto {
        self.protos.last_mut().unwrap()
    }
    pub fn gen_amir(&mut self, thir: &TypedHir) -> Result<AmirId, GenAmirError> {
        self.start_amir();
        let var = self.gen_stmt(thir)?;
        Ok(self.end_amir(var, thir.ty.clone()))
    }
    pub fn gen_stmt(&mut self, thir: &TypedHir) -> Result<VarId, GenAmirError> {
        let TypedHir { id: _, ty, expr } = thir;
        let var_id = match expr {
            thir::Expr::Literal(literal) => {
                let const_value = match literal {
                    thir::Literal::Int(int) => AStmt::Const(Const::Int(*int)),
                    thir::Literal::String(string) => AStmt::Const(Const::String(string.clone())),
                    thir::Literal::Float(a) => AStmt::Const(Const::Float(*a)),
                    thir::Literal::Rational(a, b) => AStmt::Const(Const::Rational(*a, *b)),
                };
                self.amir().bind_stmt(ty.clone(), const_value)
            }
            thir::Expr::Array(values) => {
                let values = values
                    .iter()
                    .map(|value| self.gen_stmt(value))
                    .collect::<Result<Vec<_>, _>>()?;
                self.amir().bind_stmt(ty.clone(), AStmt::Array(values))
            }
            thir::Expr::Set(values) => {
                let values = values
                    .iter()
                    .map(|value| self.gen_stmt(value))
                    .collect::<Result<Vec<_>, _>>()?;
                self.amir().bind_stmt(ty.clone(), AStmt::Set(values))
            }
            thir::Expr::Let { definition, body } => {
                self.amir().begin_scope();

                // gen definition
                let def_var = self.gen_stmt(definition)?;
                // make it named
                self.amir().create_named_var(def_var, definition.ty.clone());
                // gen body
                let var = self.gen_stmt(&**body)?;

                self.amir().end_scope(var)
            }
            thir::Expr::Perform(input) => {
                let var = self.gen_stmt(input)?;
                self.amir().bind_stmt(ty.clone(), AStmt::Perform(var))
            }
            thir::Expr::Handle {
                input,
                output,
                handler,
                expr,
            } => todo!(),
            thir::Expr::Reference => self
                .amir()
                .find_var(ty)
                .ok_or(GenAmirError::ReferencesUnknownVar(ty.clone()))?,
            thir::Expr::Op {
                op,
                operands: arguments,
            } => match op {
                thir::BuiltinOp::And => {
                    todo!()
                }
                thir::BuiltinOp::Or => {
                    todo!()
                }
                op => {
                    let arguments = arguments
                        .iter()
                        .map(|arg| self.gen_stmt(arg))
                        .collect::<Result<Vec<_>, _>>()?;
                    self.amir().bind_stmt(
                        ty.clone(),
                        AStmt::Op {
                            op: into_op::into_op(op),
                            operands: arguments,
                        },
                    )
                }
            },
            thir::Expr::Apply {
                function,
                arguments,
            } => {
                let function = self.amir().find_var(function).unwrap_or_else(|| {
                    let link = self.amir().request_link(function.clone());
                    self.amir()
                        .bind_stmt(function.clone(), AStmt::Fn(FnRef::Link(link)))
                });
                let arguments = arguments
                    .iter()
                    .map(|arg| self.gen_stmt(arg))
                    .collect::<Result<Vec<_>, _>>()?;
                self.amir().bind_stmt(
                    ty.clone(),
                    AStmt::Apply {
                        function,
                        arguments,
                    },
                )
            }
            thir::Expr::Product(values) => {
                let values = values
                    .iter()
                    .map(|value| self.gen_stmt(value))
                    .collect::<Result<Vec<_>, _>>()?;
                self.amir().bind_stmt(ty.clone(), AStmt::Product(values))
            }
            thir::Expr::Function { parameters, body } => {
                // Begin new mir
                self.start_amir();

                let var = self.gen_stmt(body)?;

                // Out of function
                let amir_id = self.end_amir(var, body.ty.clone());

                self.amir()
                    .bind_stmt(ty.clone(), AStmt::Fn(FnRef::Amir(amir_id)))
            }
            thir::Expr::Match { input, cases } => {
                let sum_type = Type::sum(cases.iter().map(|c| c.ty.clone()).collect());
                let input = self.gen_stmt(input)?;
                let input = self.amir().bind_stmt(sum_type, AStmt::Cast(input));
                let goal_block_id = self.amir().begin_block_defer();
                let match_result_var = self.amir().create_var(ty.clone());
                let cases: Vec<_> = cases
                    .iter()
                    .map(|thir::MatchCase { ty, expr }| {
                        self.amir().begin_block();
                        let match_case_result = self.gen_stmt(expr)?;
                        self.amir()
                            .bind_to(match_result_var, AStmt::Cast(match_case_result));
                        let case_block_id = self.amir().end_block(ATerminator::Goto(goal_block_id));
                        Ok(MatchCase {
                            ty: ty.clone(),
                            next: case_block_id,
                        })
                    })
                    .collect::<Result<_, _>>()?;
                self.amir()
                    .end_block(ATerminator::Match { var: input, cases });
                match_result_var
            }
            thir::Expr::Label { label, item: expr } => self.gen_stmt(&TypedHir {
                id: expr.id,
                ty: Type::Label {
                    label: label.clone(),
                    item: Box::new(expr.ty.clone()),
                },
                expr: expr.expr.clone(),
            })?,
        };
        Ok(var_id)
    }

    fn start_amir(&mut self) {
        self.protos.push(AmirProto::default());
    }

    fn end_amir(&mut self, var: VarId, ty: Type) -> AmirId {
        let proto = self.protos.pop().expect("amir must be started to end it");
        let id = AmirId(self.amirs.len());
        self.amirs.push(proto.into_amir(var, ty));
        id
    }
}

#[cfg(test)]
mod tests {
    use amir::{
        scope::{Scope, ScopeId},
        stmt::{ATerminator, StmtBind},
        var::AVar,
    };

    use super::*;

    #[test]
    fn simple() {
        let thir = TypedHir {
            id: 0,
            ty: Type::Number,
            expr: thir::Expr::Literal(thir::Literal::Int(1)),
        };
        let mut gen = AmirGen::default();
        gen.gen_amir(&thir).unwrap();

        assert_eq!(gen.amirs.len(), 1);
        assert_eq!(gen.amirs[0].parameters, vec![]);
        assert_eq!(gen.amirs[0].scopes, vec![Scope { super_scope: None }]);
        assert_eq!(
            gen.amirs[0].vars,
            vec![AVar {
                ty: Type::Number,
                scope: ScopeId(0)
            }]
        );
        assert_eq!(gen.amirs[0].blocks.len(), 1);
        assert_eq!(
            gen.amirs[0].blocks[0].stmts,
            vec![StmtBind {
                var: VarId(0),
                stmt: AStmt::Const(Const::Int(1)),
            }]
        );
        assert_eq!(
            gen.amirs[0].blocks[0].terminator,
            ATerminator::Return(VarId(0))
        );
    }
}