use serde::{Serialize, Deserialize};
use types::Type;

use crate::scope::ScopeId;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VarId(pub usize);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AVar<T = Type> {
    pub ty: T,
    pub scope: ScopeId,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Vars<T = Type>(pub Vec<AVar<T>>);

impl<T> Vars<T> {
    pub fn get(&self, id: &VarId) -> &AVar<T> {
        &self.0[id.0]
    }

    pub fn new_var(&mut self, scope: ScopeId, ty: T) -> VarId {
        let id = VarId(self.0.len());
        self.0.push(AVar { ty, scope });
        id
    }
}
