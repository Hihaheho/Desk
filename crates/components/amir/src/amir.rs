use types::Type;

use crate::{block::ABasicBlock, link::ALink, scope::Scope, var::AVar};

#[derive(Clone, Debug, PartialEq)]
pub struct Amir {
    pub parameters: Vec<Type>,
    pub output: Type,
    // first N items in vars are arguments.
    pub vars: Vec<AVar>,
    pub scopes: Vec<Scope>,
    pub blocks: Vec<ABasicBlock>,
    pub links: Vec<ALink>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AmirId(pub usize);
