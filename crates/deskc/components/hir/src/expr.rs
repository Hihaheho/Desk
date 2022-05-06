pub use ids::LinkName;

use crate::{meta::WithMeta, ty::Type};

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    String(String),
    Int(i64),
    Rational(i64, i64),
    Float(f64),
    Hole,
}

// Literal::Float should not be NaN
impl Eq for Literal {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Handler {
    pub input: WithMeta<Type>,
    pub output: WithMeta<Type>,
    pub handler: WithMeta<Expr>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Literal(Literal),
    Let {
        ty: WithMeta<Type>,
        definition: Box<WithMeta<Self>>,
        expression: Box<WithMeta<Self>>,
    },
    Perform {
        input: Box<WithMeta<Self>>,
        output: WithMeta<Type>,
    },
    Continue {
        input: Box<WithMeta<Self>>,
        output: Option<WithMeta<Type>>,
    },
    Handle {
        handlers: Vec<Handler>,
        expr: Box<WithMeta<Self>>,
    },
    Apply {
        function: WithMeta<Type>,
        link_name: Option<LinkName>,
        arguments: Vec<WithMeta<Self>>,
    },
    Product(Vec<WithMeta<Self>>),
    Match {
        of: Box<WithMeta<Self>>,
        cases: Vec<MatchCase>,
    },
    Typed {
        ty: WithMeta<Type>,
        item: Box<WithMeta<Self>>,
    },
    Function {
        parameter: WithMeta<Type>,
        body: Box<WithMeta<Self>>,
    },
    Array(Vec<WithMeta<Self>>),
    Set(Vec<WithMeta<Self>>),
    Label {
        label: String,
        item: Box<WithMeta<Self>>,
    },
    Brand {
        brand: String,
        item: Box<WithMeta<Self>>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchCase {
    pub ty: WithMeta<Type>,
    pub expr: WithMeta<Expr>,
}
