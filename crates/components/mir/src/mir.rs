use amir::{link::ALink, scope::Scope, var::AVar};

use crate::{
    stmt::Stmt,
    ty::{ConcType, ConcTypeId},
};

pub type Var = AVar<ConcType>;
pub type BasicBlock = amir::block::ABasicBlock<Stmt>;
pub type Link = ALink<ConcType>;

pub struct Mir {
    pub parameters: Vec<ConcType>,
    pub output: ConcType,
    pub vars: Vec<Var>,
    pub scopes: Vec<Scope>,
    pub blocks: Vec<BasicBlock>,
    pub links: Vec<Link>,
}
