use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DProcessId(pub Uuid);

impl DProcessId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}
