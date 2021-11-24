use mir::VarId;

use crate::{value::Value, EvalMir};

impl EvalMir {
    pub(crate) fn get_var(&self, id: &VarId) -> &Value {
        self.registers.get(id).unwrap()
    }
}
