use serde::{Deserialize, Serialize};
use ty::Type;

use crate::stmt::{Stmt, StmtBind, Terminator};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BasicBlock<S = Stmt, T = Type> {
    pub stmts: Vec<StmtBind<S>>,
    pub terminator: Terminator<T>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct BlockId(pub usize);
