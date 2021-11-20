use crate::{meta::WithMeta, ty::Type};

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    String(String),
    Int(i64),
    Rational(i64, i64),
    Float(f64),
    Hole,
}

#[derive(Clone, Debug, PartialEq)]
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
    Handle {
        input: WithMeta<Type>,
        output: WithMeta<Type>,
        handler: Box<WithMeta<Self>>,
        expr: Box<WithMeta<Self>>,
    },
    Apply {
        function: WithMeta<Type>,
        arguments: Vec<WithMeta<Self>>,
    },
    Product(Vec<WithMeta<Self>>),
    Match {
        of: Box<WithMeta<Self>>,
        cases: Vec<MatchCase>,
    },
    Typed {
        ty: WithMeta<Type>,
        expr: Box<WithMeta<Self>>,
    },
    Function {
        parameter: WithMeta<Type>,
        body: Box<WithMeta<Self>>,
    },
    Array(Vec<WithMeta<Self>>),
    Set(Vec<WithMeta<Self>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct MatchCase {
    pub ty: WithMeta<Type>,
    pub expr: WithMeta<Expr>,
}
