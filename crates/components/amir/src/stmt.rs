use types::Type;

use crate::{amir::AmirId, block::BlockId, link::LinkId, var::VarId};

#[derive(Clone, Debug, PartialEq)]
pub struct StmtBind<T = AStmt> {
    pub var: VarId,
    pub stmt: T,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AStmt {
    Const(Const),
    Product(Vec<VarId>),
    Array(Vec<VarId>),
    Set(Vec<VarId>),
    Fn(FnRef),
    Perform(VarId),
    MatchResult(VarId),
    // TODO: Handle
    Apply {
        function: VarId,
        arguments: Vec<VarId>,
    },
    Op {
        op: Op,
        operands: Vec<VarId>,
    },
    Cast(VarId),
    Parameter,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FnRef {
    Amir(AmirId),
    Link(LinkId),
    Clojure(AmirId, Vec<VarId>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Const {
    Int(i64),
    Rational(i64, i64),
    Float(f64),
    String(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Mod,
    Pow,
    Eq,
    Neq,
    Lt,
    Le,
    Gt,
    Ge,
    Not,
    Neg,
    Pos,
    Shl,
    Shr,
    BitAnd,
    BitOr,
    BitXor,
    BitNot,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MatchCase<T = Type> {
    pub ty: T,
    pub next: BlockId,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ATerminator<T = Type> {
    Return(VarId),
    Match {
        var: VarId,
        cases: Vec<MatchCase<T>>,
    },
    Goto(BlockId),
}
