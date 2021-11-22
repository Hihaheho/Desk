use amir::{
    stmt::{Const, FnRef, Op},
    var::VarId,
};

pub enum Stmt {
    Const(Const),
    Product(Vec<VarId>),
    Array(Vec<VarId>),
    Set(Vec<VarId>),
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
    Ref(VarId),
    RefMut(VarId),
}
