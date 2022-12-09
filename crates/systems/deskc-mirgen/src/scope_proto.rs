use std::collections::HashMap;

use mir::{
    scope::{Scope, ScopeId},
    var::VarId,
};
use ty::Type;

#[derive(Debug, Default)]
pub struct ScopeProto {
    pub super_id: Option<ScopeId>,
    // Only used in mir generation
    pub named_vars: HashMap<Type, VarId>,
}

impl ScopeProto {
    pub fn into_scope(self) -> Scope {
        Scope {
            super_scope: self.super_id,
        }
    }
}
