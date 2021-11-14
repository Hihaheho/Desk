use uuid::Uuid;

use crate::{meta::WithMeta, ty::Type};

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    String(String),
    Int(i64),
    Rational(i64, i64),
    Float(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Handler {
    pub ty: WithMeta<Type>,
    pub expr: WithMeta<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Let {
        definition: Box<WithMeta<Self>>,
        expression: Box<WithMeta<Self>>,
    },
    Perform {
        effect: Box<WithMeta<Self>>,
    },
    Effectful {
        class: WithMeta<Type>,
        expr: Box<WithMeta<Self>>,
        handlers: Vec<Handler>,
    },
    Call {
        function: WithMeta<Type>,
        arguments: Vec<WithMeta<Self>>,
    },
    Product(Vec<WithMeta<Self>>),
    Typed {
        ty: WithMeta<Type>,
        expr: Box<WithMeta<Self>>,
    },
    Hole,
    Function(Box<WithMeta<Self>>),
    Array(Vec<WithMeta<Self>>),
    Set(Vec<WithMeta<Self>>),
}
