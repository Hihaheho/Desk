use std::collections::HashMap;

use mir::{
    ty::{ConcEffect, ConcType},
    MirId,
};

use crate::eval_mir::Handler;

#[cfg_attr(feature = "withserde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Int(i64),
    Float(f64),
    Rational(i64, i64),
    Tuple(Vec<Value>),
    Variant { id: usize, value: Box<Value> },
    FnRef(FnRef),
}

#[cfg_attr(feature = "withserde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum FnRef {
    Link(ConcType),
    Closure(Closure),
    Recursion,
}

#[cfg_attr(feature = "withserde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Closure {
    pub mir: MirId,
    pub captured: HashMap<ConcType, Value>,
    pub handlers: HashMap<ConcEffect, Handler>,
}
