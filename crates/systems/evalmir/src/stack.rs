use std::collections::HashMap;

use mir::{ty::ConcType, BlockId, VarId};

use crate::value::Value;

#[derive(Debug, Clone)]
pub struct StackItem {
    pub registers: HashMap<VarId, Value>,
    pub parameters: HashMap<ConcType, Value>,
    pub pc_block: BlockId,
    pub pc_stmt_idx: usize,
}

impl StackItem {
    pub fn load_value(&self, var: &VarId) -> &Value {
        self.registers.get(var).unwrap()
    }

    pub fn store_value(&mut self, var: VarId, value: Value) {
        self.registers.insert(var, value);
    }
}
