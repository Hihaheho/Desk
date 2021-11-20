use types::Type;

use crate::{FnId, block::BlockId, var::VarId};

#[derive(Clone, Debug, PartialEq)]
pub struct StmtBind {
    pub var: VarId,
    pub stmt: Stmt,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Const(Const),
    Product(Vec<VarId>),
    Array(Vec<VarId>),
    Set(Vec<VarId>),
    Fn(FnId),
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
}

#[derive(Clone, Debug, PartialEq)]
pub struct MatchCase {
	pub ty: Type,
	pub next: BlockId,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Terminator {
	Exit,
	Match {
		var: VarId,
		cases: Vec<MatchCase>,
	}
}
