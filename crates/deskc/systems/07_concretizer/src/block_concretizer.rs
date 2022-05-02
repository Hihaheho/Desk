mod cast;

use amir::{
    block::ABasicBlock,
    stmt::{AStmt, MatchCase},
};
use mir::{
    mir::BasicBlock,
    stmt::{FnRef, Stmt},
    ty::ConcType,
    ATerminator, StmtBind, Vars,
};
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
        blocks: &[ABasicBlock<AStmt, Type>],
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
            let mut bind = |stmt| self.stmts.push(StmtBind { var: *var, stmt });
            match stmt {
                AStmt::Const(value) => bind(Stmt::Const(value.clone())),
                AStmt::Product(values) => bind(Stmt::Tuple(values.to_vec())),
                AStmt::Array(values) => bind(Stmt::Array(values.to_vec())),
                AStmt::Set(values) => bind(Stmt::Set(values.to_vec())),
                AStmt::Fn(fn_ref) => match fn_ref {
                    amir::stmt::FnRef::Link(link) => bind(Stmt::Fn(FnRef::Link(
                        self.type_concretizer.gen_conc_type(link),
                    ))),
                    amir::stmt::FnRef::Closure {
                        amir,
                        captured,
                        handlers,
                    } => bind(Stmt::Fn(FnRef::Clojure {
                        amir: *amir,
                        captured: captured.clone(),
                        handlers: handlers
                            .iter()
                            .map(|(effect, handler)| {
                                (self.type_concretizer.gen_conc_effect(effect), *handler)
                            })
                            .collect(),
                    })),
                },
                AStmt::Perform(var) => bind(Stmt::Perform(*var)),
                AStmt::Apply {
                    function,
                    arguments,
                } => bind(Stmt::Apply {
                    function: *function,
                    arguments: arguments.to_vec(),
                }),
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
                AStmt::Parameter => bind(Stmt::Parameter),
                AStmt::Recursion => bind(Stmt::Recursion),
                AStmt::Link(name) => bind(Stmt::Link(name.clone())),
            };
        }
        match terminator {
            ATerminator::Return(var) => ATerminator::Return(*var),
            ATerminator::Match { var, cases } => {
                let def = self.enum_defs.get_enum_def(
                    self.type_concretizer
                        .gen_conc_type(&Type::sum(cases.iter().map(|c| c.ty.clone()).collect())),
                );
                ATerminator::<usize>::Match {
                    var: *var,
                    cases: cases
                        .iter()
                        .map(|MatchCase { ty, next }| MatchCase {
                            ty: def.get_variant_index(self.type_concretizer.gen_conc_type(ty)),
                            next: *next,
                        })
                        .collect(),
                }
            }
            ATerminator::Goto(block_id) => ATerminator::Goto(*block_id),
        }
    }
}
