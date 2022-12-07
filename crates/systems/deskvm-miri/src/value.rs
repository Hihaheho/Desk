use std::collections::HashMap;

use mir::mir::ControlFlowGraphId;
use types::{Effect, Type};

use crate::eval_cfg::{EvalCfg, Handler};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    // empty product
    Unit,
    String(String),
    Int(i64),
    Float(f64),
    // b must be unsigned to avoid ambiguity.
    Rational(i64, u64),
    Product(HashMap<Type, Value>),
    Variant { ty: Type, value: Box<Value> },
    Vector(Vec<Self>),
    FnRef(FnRef),
    TraitObject { ty: Type, value: Box<Value> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum FnRef {
    Link(Type),
    Closure(Closure),
    Recursion,
    Operator(fn(EvalCfg) -> OperatorOutput),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Closure {
    pub mir: ControlFlowGraphId,
    pub captured: HashMap<Type, Value>,
    pub handlers: HashMap<Effect, Handler>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OperatorOutput {
    Return(Value),
    Perform { effect: Effect, input: Value },
}
