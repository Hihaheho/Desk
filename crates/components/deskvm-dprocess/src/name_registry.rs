use std::collections::HashMap;

use crate::dprocess::DProcessId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NameRegistry {
    pub names: HashMap<String, DProcessId>,
}
