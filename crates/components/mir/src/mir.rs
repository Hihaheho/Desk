use crate::{block::BasicBlock, scope::Scope, var::Var};

#[derive(Clone, Debug, PartialEq)]
pub struct Mir {
    pub vars: Vec<Var>,
    pub scopes: Vec<Scope>,
    pub blocks: Vec<BasicBlock>,
}
