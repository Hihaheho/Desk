use std::collections::HashMap;

use amir::{
    scope::{Scope, ScopeId},
    var::VarId,
};
use types::Type;

#[derive(Debug, Default)]
pub struct ScopeProto {
    pub super_id: Option<ScopeId>,
    pub named_vars: HashMap<Type, VarId>,
}

impl ScopeProto {
    pub fn into_scope(self) -> Scope {
        Scope {
            super_scope: self.super_id,
        }
    }
}
