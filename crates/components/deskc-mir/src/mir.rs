use serde::{Deserialize, Serialize};
use ty::Type;

use crate::{block::BasicBlock, scope::Scope, stmt::LinkId, var::Vars};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControlFlowGraph {
    // function parameter
    pub parameter: Option<Type>,
    // implicit parameters that captured from outer scope.
    pub captured: Vec<Type>,
    pub output: Type,
    // first N items in vars are arguments.
    pub vars: Vars,
    pub scopes: Vec<Scope>,
    pub blocks: Vec<BasicBlock>,
    pub links: Vec<LinkId>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ControlFlowGraphId(pub usize);

#[derive(Clone, Debug, PartialEq, Eq)]
/// Mir -> CFGs -> Blocks -> Stmts
pub struct Mir {
    pub entrypoint: ControlFlowGraphId,
    pub cfgs: Vec<ControlFlowGraph>,
}

impl ControlFlowGraph {
    pub fn get_type(&self) -> &Type {
        &self.output
    }
}

impl Mir {
    pub fn captured(&self) -> &[Type] {
        &self.cfgs[self.entrypoint.0].captured
    }
}
