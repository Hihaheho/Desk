use std::collections::HashMap;

use amir::{
    amir::ControlFlowGraphId,
    stmt::{Const, Op},
    var::VarId,
};
use conc_types::{ConcEffect, ConcType};
use ids::LinkName;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FnRef {
    Link(ConcType),
    Clojure {
        amir: ControlFlowGraphId,
        captured: Vec<VarId>,
        handlers: HashMap<ConcEffect, VarId>,
    },
}
