use crate::stmt::Stmt;
use amir::{
    amir::ControlFlowGraphId,
    scope::Scope,
    var::{AVar, Vars},
};
use conc_types::ConcType;
use serde::{Deserialize, Serialize};

pub type Var = AVar<ConcType>;
pub type BasicBlock = amir::block::ABasicBlock<Stmt, usize>;
pub type LinkId = ids::LinkId<ConcType>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControlFlowGraph {
    pub parameters: Vec<ConcType>,
    pub captured: Vec<ConcType>,
    pub output: ConcType,
    pub vars: Vars<ConcType>,
    pub scopes: Vec<Scope>,
    pub blocks: Vec<BasicBlock>,
    pub links: Vec<LinkId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mir {
    pub entrypoint: ControlFlowGraphId,
    pub cfgs: Vec<ControlFlowGraph>,
}
