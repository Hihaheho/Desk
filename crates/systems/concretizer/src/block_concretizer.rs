mod cast;

use amir::{
    block::ABasicBlock,
    stmt::{AStmt, MatchCase},
};
use mir::{mir::BasicBlock, stmt::Stmt, ty::ConcType, ATerminator, Const, StmtBind, Vars};
use types::Type;

use crate::{enumdef::EnumDefs, type_concretizer::TypeConcretizer};

pub struct BlockConcretizer<'a> {
    pub enum_defs: &'a mut EnumDefs,
    pub type_concretizer: &'a mut TypeConcretizer,
    pub vars: &'a mut Vars<ConcType>,
    pub stmts: Vec<StmtBind<Stmt>>,
}
impl<'a> BlockConcretizer<'a> {
    pub(crate) fn concretize_blocks(
        &mut self,
        blocks: &Vec<ABasicBlock<AStmt, Type>>,
    ) -> Vec<BasicBlock> {
        blocks
            .iter()
            .map(|block| {
                let terminator = self.concretize_block(block);
                let stmts = self.stmts.drain(0..self.stmts.len()).collect();
                ABasicBlock { stmts, terminator }
            })
            .collect()
    }

    fn concretize_block(
        &mut self,
        ABasicBlock { stmts, terminator }: &ABasicBlock<AStmt, Type>,
    ) -> ATerminator<usize> {
        for StmtBind { stmt, var } in stmts {
            let var_data = self.vars.get(var);
            let stmt = match stmt {
                AStmt::Const(value) => self.stmts.push(StmtBind {
                    var: *var,
                    stmt: Stmt::Const(value.clone()),
                }),
                AStmt::Product(values) => self.stmts.push(StmtBind {
                    var: *var,
                    stmt: Stmt::Tuple(values.iter().cloned().collect()),
                }),
                AStmt::Array(values) => self.stmts.push(StmtBind {
                    var: *var,
                    stmt: Stmt::Array(values.iter().cloned().collect()),
                }),
                AStmt::Set(values) => self.stmts.push(StmtBind {
                    var: *var,
                    stmt: Stmt::Set(values.iter().cloned().collect()),
                }),
                AStmt::Fn(fn_ref) => self.stmts.push(StmtBind {
                    var: *var,
                    stmt: Stmt::Fn(*fn_ref),
                }),
                AStmt::Perform(var) => self.stmts.push(StmtBind {
                    var: *var,
                    stmt: Stmt::Perform(*var),
                }),
                AStmt::Apply {
                    function,
                    arguments,
                } => todo!(),
                AStmt::Op { op, operands } => self.stmts.push(StmtBind {
                    var: *var,
                    stmt: Stmt::Op {
                        op: op.clone(),
                        operands: operands.clone(),
                    },
                }),
                AStmt::MatchResult(var) => {
                    // TODO make tuple if var's type is sum type.
                    self.stmts.push(StmtBind {
                        var: *var,
                        stmt: Stmt::Move(*var),
                    });
                }
                AStmt::Cast(from) => self.cast_to(*var, *from),
            };
        }
        match terminator {
            ATerminator::Return(var) => ATerminator::Return(*var),
            ATerminator::Match { var, cases } => {
                let def = self.enum_defs.get_enum_def(dbg!(self
                    .type_concretizer
                    .to_conc_type(&Type::sum(cases.iter().map(|c| c.ty.clone()).collect(),))));
                ATerminator::<usize>::Match {
                    var: *var,
                    cases: cases
                        .iter()
                        .map(|MatchCase { ty, next }| MatchCase {
                            ty: def
                                .get_variant_index(self.type_concretizer.to_conc_type(ty).clone()),
                            next: *next,
                        })
                        .collect(),
                }
            }
            ATerminator::Goto(block_id) => ATerminator::Goto(*block_id),
        }
    }
}