use types::Type;

use crate::stmt::{AStmt, ATerminator, StmtBind};

#[cfg_attr(feature = "withserde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct ABasicBlock<S = AStmt, T = Type> {
    pub stmts: Vec<StmtBind<S>>,
    pub terminator: ATerminator<T>,
}

#[cfg_attr(feature = "withserde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct BlockId(pub usize);
