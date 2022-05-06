use crate::{stmt::Stmt, ty::ConcType};
use amir::{
    amir::AmirId,
    scope::Scope,
    var::{AVar, Vars},
};

pub type Var = AVar<ConcType>;
pub type BasicBlock = amir::block::ABasicBlock<Stmt, usize>;
pub type LinkId = ids::LinkId<ConcType>;

#[cfg_attr(feature = "withserde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mir {
    pub parameters: Vec<ConcType>,
    pub captured: Vec<ConcType>,
    pub output: ConcType,
    pub vars: Vars<ConcType>,
    pub scopes: Vec<Scope>,
    pub blocks: Vec<BasicBlock>,
    pub links: Vec<LinkId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mirs {
    pub entrypoint: AmirId,
    pub mirs: Vec<Mir>,
}
