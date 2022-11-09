use serde::{Deserialize, Serialize};
use types::Type;

use crate::stmt::{AStmt, ATerminator, StmtBind};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ABasicBlock<S = AStmt, T = Type> {
    pub stmts: Vec<StmtBind<S>>,
    pub terminator: ATerminator<T>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct BlockId(pub usize);
