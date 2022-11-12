use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ScopeId(pub usize);

#[derive(Clone, Debug, PartialEq, Default, Eq, Serialize, Deserialize)]
pub struct Scope {
    pub super_scope: Option<ScopeId>,
}
