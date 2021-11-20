#[derive(Clone, Debug, PartialEq)]
pub struct ScopeId(pub usize);

#[derive(Clone, Debug, PartialEq)]
pub struct Scope {
    pub super_scope: Option<Box<ScopeId>>,
}
