use crate::stmt::{StmtBind, Terminator};

#[derive(Clone, Debug, PartialEq)]
pub struct BasicBlock {
    pub statements: Vec<StmtBind>,
    pub terminator: Option<Terminator>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BlockId(pub usize);
