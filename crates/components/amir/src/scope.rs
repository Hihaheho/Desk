#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ScopeId(pub usize);

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Scope {
    pub super_scope: Option<ScopeId>,
}
