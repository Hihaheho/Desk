use dson::Dson;
pub use ids::LinkName;

use crate::{
    meta::WithMeta,
    ty::{Effect, Type},
};

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    String(String),
    Integer(i64),
    // b must be unsigned to avoid ambiguity.
    Rational(i64, u64),
    Real(f64),
}

// Literal::Real should not be NaN
impl Eq for Literal {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Handler {
    pub effect: Effect,
    pub handler: WithMeta<Expr>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Literal(Literal),
    Do {
        stmt: Box<WithMeta<Self>>,
        expr: Box<WithMeta<Self>>,
    },
    Let {
        definition: Box<WithMeta<Self>>,
        expr: Box<WithMeta<Self>>,
    },
    Perform {
        input: Box<WithMeta<Self>>,
        output: WithMeta<Type>,
    },
    Continue {
        input: Box<WithMeta<Self>>,
        output: WithMeta<Type>,
    },
    Handle {
        handlers: Vec<WithMeta<Handler>>,
        expr: Box<WithMeta<Self>>,
    },
    Apply {
        function: WithMeta<Type>,
        link_name: LinkName,
        arguments: Vec<WithMeta<Self>>,
    },
    Product(Vec<WithMeta<Self>>),
    Match {
        of: Box<WithMeta<Self>>,
        cases: Vec<WithMeta<MatchCase>>,
    },
    Typed {
        ty: WithMeta<Type>,
        item: Box<WithMeta<Self>>,
    },
    Function {
        parameter: WithMeta<Type>,
        body: Box<WithMeta<Self>>,
    },
    Vector(Vec<WithMeta<Self>>),
    Map(Vec<WithMeta<MapElem>>),
    Label {
        label: Dson,
        item: Box<WithMeta<Self>>,
    },
    Brand {
        brand: Dson,
        item: Box<WithMeta<Self>>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchCase {
    pub ty: WithMeta<Type>,
    pub expr: WithMeta<Expr>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MapElem {
    pub key: WithMeta<Expr>,
    pub value: WithMeta<Expr>,
}
