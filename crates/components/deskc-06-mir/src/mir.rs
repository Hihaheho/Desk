use crate::{stmt::Stmt, ty::ConcType};
use amir::{
    amir::ControlFlowGraphId,
    scope::Scope,
    var::{AVar, Vars},
};

pub type Var = AVar<ConcType>;
pub type BasicBlock = amir::block::ABasicBlock<Stmt, usize>;
pub type LinkId = ids::LinkId<ConcType>;

#[cfg_attr(feature = "withserde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub mirs: Vec<ControlFlowGraph>,
}
