use std::collections::HashMap;

use language::{semantic::ir::OperatorId, Operator};

pub struct OperatorDefinitions {
    pub map: HashMap<OperatorId, Box<dyn Operator + Send + Sync>>,
}
