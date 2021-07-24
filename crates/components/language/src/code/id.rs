use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct CodeId(Uuid);

impl ToString for CodeId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl CodeId {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}
