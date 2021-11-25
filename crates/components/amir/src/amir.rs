use types::Type;

use crate::{block::ABasicBlock, link::ALink, scope::Scope, var::AVar};

#[derive(Clone, Debug, PartialEq)]
pub struct Amir {
    // function parameters
    pub parameters: Vec<Type>,
    // implicit parameters that captured from outer scope.
    pub captured: Vec<Type>,
    pub output: Type,
    // first N items in vars are arguments.
    pub vars: Vec<AVar>,
    pub scopes: Vec<Scope>,
    pub blocks: Vec<ABasicBlock>,
    pub links: Vec<ALink>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AmirId(pub usize);
