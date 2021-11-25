use amir::{
    stmt::{Const, FnRef, Op},
    var::VarId,
};

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
    Returned,
}
