use types::Type;

use crate::stmt::{AStmt, StmtBind, Terminator};

#[derive(Clone, Debug, PartialEq)]
pub struct ABasicBlock<S = AStmt, T = Type> {
    pub stmts: Vec<StmtBind<S>>,
    pub terminator: Terminator<T>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlockId(pub usize);
