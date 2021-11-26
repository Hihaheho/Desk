use std::collections::HashMap;

use mir::{ty::ConcType, LinkId, MirId};

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
    Mir(MirId),
    Link(LinkId),
    Closure {
        mir: MirId,
        captured: HashMap<ConcType, Value>,
    },
}
