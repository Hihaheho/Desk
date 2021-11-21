use crate::stmt::{StmtBind, Terminator};

#[derive(Clone, Debug, PartialEq)]
pub struct BasicBlock {
    pub stmts: Vec<StmtBind>,
    pub terminator: Terminator,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BlockId(pub usize);
