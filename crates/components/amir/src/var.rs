use types::Type;

use crate::scope::ScopeId;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct VarId(pub usize);

#[derive(Clone, Debug, PartialEq)]
pub struct AVar<T = Type> {
    pub ty: T,
    pub scope: ScopeId,
}
