use std::collections::HashMap;

use mir::mir::ControlFlowGraphId;
use serde::{Deserialize, Serialize};
use types::{Effect, Type};

use crate::eval_cfg::Handler;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Value {
    // empty product
    Unit,
    String(String),
    Int(i64),
    Float(f64),
    Rational(i64, i64),
    Product(HashMap<Type, Value>),
    Variant { ty: Type, value: Box<Value> },
    FnRef(FnRef),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FnRef {
    Link(Type),
    Closure(Closure),
    Recursion,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Closure {
    pub mir: ControlFlowGraphId,
    pub captured: HashMap<Type, Value>,
    pub handlers: HashMap<Effect, Handler>,
}
