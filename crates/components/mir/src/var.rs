use types::Type;

use crate::scope::ScopeId;

#[derive(Clone, Debug, PartialEq)]
pub struct VarId(pub usize);

#[derive(Clone, Debug, PartialEq)]
pub struct Var {
    pub ty: Type,
    pub scope: ScopeId,
}
