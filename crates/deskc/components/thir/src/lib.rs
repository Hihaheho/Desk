pub use link::LinkName;
use types::{Effect, Type};

pub type Id = usize;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Int(i64),
    Float(f64),
    Rational(i64, i64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchCase {
    pub ty: Type,
    pub expr: TypedHir,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypedHir {
    pub id: Id,
    pub ty: Type,
    pub expr: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Handler {
    pub effect: Effect,
    pub handler: TypedHir,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Match {
        input: Box<TypedHir>,
        cases: Vec<MatchCase>,
    },
    Let {
        definition: Box<TypedHir>,
        body: Box<TypedHir>,
    },
    Perform(Box<TypedHir>),
    Handle {
        handlers: Vec<Handler>,
        expr: Box<TypedHir>,
    },
    Op {
        op: BuiltinOp,
        operands: Vec<TypedHir>,
    },
    Apply {
        function: Type,
        link_name: Option<LinkName>,
        arguments: Vec<TypedHir>,
    },
    Product(Vec<TypedHir>),
    Function {
        parameters: Vec<Type>,
        body: Box<TypedHir>,
    },
    Array(Vec<TypedHir>),
    Set(Vec<TypedHir>),
    Label {
        label: String,
        item: Box<TypedHir>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BuiltinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Mod,
    Eq,
    Neq,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    Not,
    Neg,
    BitAnd,
    BitOr,
    BitXor,
    BitNot,
    Shl,
    Shr,
}
