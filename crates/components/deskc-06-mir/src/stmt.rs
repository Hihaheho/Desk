use std::collections::HashMap;

use amir::{
    amir::ControlFlowGraphId,
    stmt::{Const, Op},
    var::VarId,
};
use conc_types::{ConcEffect, ConcType};
use ids::LinkName;

#[cfg_attr(feature = "withserde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
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
    Link(LinkName),
}

#[cfg_attr(feature = "withserde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FnRef {
    Link(ConcType),
    Clojure {
        amir: ControlFlowGraphId,
        captured: Vec<VarId>,
        handlers: HashMap<ConcEffect, VarId>,
    },
}
