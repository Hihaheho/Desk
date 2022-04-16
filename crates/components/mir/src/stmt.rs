use std::collections::HashMap;

use amir::{
    amir::AmirId,
    stmt::{Const, Op},
    var::VarId,
};

use crate::ty::{ConcEffect, ConcType};

#[cfg_attr(feature = "withserde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Const(Const),
    Tuple(Vec<VarId>),
    Array(Vec<VarId>),
    Set(Vec<VarId>),
    Index {
        tuple: VarId,
        index: usize,
    },
    Fn(FnRef),
    Perform(VarId),
    // TODO: Handle
    Apply {
        function: VarId,
        arguments: Vec<VarId>,
    },
    Op {
        op: Op,
        operands: Vec<VarId>,
    },
    Variant {
        id: usize,
        value: VarId,
    },
    Move(VarId),
    Ref(VarId),
    RefMut(VarId),
    Parameter,
    Recursion,
}

#[cfg_attr(feature = "withserde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum FnRef {
    Link(ConcType),
    Clojure {
        amir: AmirId,
        captured: Vec<VarId>,
        handlers: HashMap<ConcEffect, VarId>,
    },
}
