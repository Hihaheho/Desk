use types::Type;

use crate::stmt::{AStmt, StmtBind, ATerminator};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct ABasicBlock<S = AStmt, T = Type> {
    pub stmts: Vec<StmtBind<S>>,
    pub terminator: ATerminator<T>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlockId(pub usize);
