use amir::{
    link::ALink,
    scope::Scope,
    var::{AVar, Vars},
};

use crate::{stmt::Stmt, ty::ConcType};

pub type Var = AVar<ConcType>;
pub type BasicBlock = amir::block::ABasicBlock<Stmt, usize>;
pub type Link = ALink<ConcType>;

#[derive(Debug, Clone, PartialEq)]
pub struct Mir {
    pub parameters: Vec<ConcType>,
    pub captured: Vec<ConcType>,
    pub output: ConcType,
    pub vars: Vars<ConcType>,
    pub scopes: Vec<Scope>,
    pub blocks: Vec<BasicBlock>,
    pub links: Vec<Link>,
}
