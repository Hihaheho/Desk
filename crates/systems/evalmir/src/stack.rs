use std::collections::HashMap;

use types::Type;

use crate::value::Value;

#[derive(Clone, Debug, Default)]
pub struct Stack {
    pub storage: HashMap<Type, Value>,
}

impl Stack {
    pub fn get<'a>(&'a self, ty: &Type) -> Option<&'a Value> {
        self.storage.get(ty)
    }
}
