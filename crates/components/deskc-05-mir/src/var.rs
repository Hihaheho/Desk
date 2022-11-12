use serde::{Deserialize, Serialize};
use types::Type;

use crate::scope::ScopeId;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VarId(pub usize);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Var<T = Type> {
    pub ty: T,
    pub scope: ScopeId,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Vars<T = Type>(pub Vec<Var<T>>);

impl<T> Vars<T> {
    pub fn get(&self, id: &VarId) -> &Var<T> {
        &self.0[id.0]
    }

    pub fn get_mut(&mut self, id: &VarId) -> &mut Var<T> {
        &mut self.0[id.0]
    }

    pub fn add_new_var(&mut self, scope: ScopeId, ty: T) -> VarId {
        let id = VarId(self.0.len());
        self.0.push(Var { ty, scope });
        id
    }
}
