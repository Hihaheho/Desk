use types::Type;

use crate::stmt::{AStmt, StmtBind, Terminator};

#[derive(Clone, Debug, PartialEq)]
pub struct ABasicBlock<S = AStmt> {
    pub stmts: Vec<StmtBind<S>>,
    pub terminator: Terminator,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BlockId(pub usize);
