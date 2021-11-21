use types::Type;

use crate::{block::BasicBlock, link::Link, scope::Scope, var::Var};

#[derive(Clone, Debug, PartialEq)]
pub struct Amir {
    pub parameters: Vec<Type>,
    pub output: Type,
    // first N items in vars are arguments.
    pub vars: Vec<Var>,
    pub scopes: Vec<Scope>,
    pub blocks: Vec<BasicBlock>,
    pub links: Vec<Link>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AmirId(pub usize);
