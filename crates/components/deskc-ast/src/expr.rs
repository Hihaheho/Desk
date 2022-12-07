use dson::Dson;
use ids::CardId;
pub use ids::LinkName;

use crate::{span::WithSpan, ty::Type};

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    String(String),
    Integer(i64),
    // b must be unsigned to avoid ambiguity.
    Rational(i64, u64),
    Float(f64),
}

// Literal::Float should not be NaN
impl Eq for Literal {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Handler {
    pub input: WithSpan<Type>,
    pub output: WithSpan<Type>,
    pub handler: WithSpan<Expr>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Literal(Literal),
    Do {
        stmt: Box<WithSpan<Self>>,
        expr: Box<WithSpan<Self>>,
    },
    Let {
        definition: Box<WithSpan<Self>>,
        body: Box<WithSpan<Self>>,
    },
    Perform {
        input: Box<WithSpan<Self>>,
        output: WithSpan<Type>,
    },
    Continue {
        input: Box<WithSpan<Self>>,
        output: WithSpan<Type>,
    },
    Handle {
        expr: Box<WithSpan<Self>>,
        handlers: Vec<Handler>,
    },
    Apply {
        function: WithSpan<Type>,
        link_name: LinkName,
        arguments: Vec<WithSpan<Self>>,
    },
    Product(Vec<WithSpan<Self>>),
    Match {
        of: Box<WithSpan<Self>>,
        cases: Vec<MatchCase>,
    },
    Typed {
        ty: WithSpan<Type>,
        item: Box<WithSpan<Self>>,
    },
    Hole,
    Function {
        parameter: WithSpan<Type>,
        body: Box<WithSpan<Self>>,
    },
    Vector(Vec<WithSpan<Self>>),
    Map(Vec<MapElem>),
    Attributed {
        attr: Dson,
        item: Box<WithSpan<Self>>,
    },
    Brand {
        brand: Dson,
        item: Box<WithSpan<Self>>,
    },
    Label {
        label: Dson,
        item: Box<WithSpan<Self>>,
    },
    NewType {
        ident: String,
        ty: WithSpan<Type>,
        expr: Box<WithSpan<Self>>,
    },
    Comment {
        text: String,
        item: Box<WithSpan<Self>>,
    },
    Card {
        id: CardId,
        item: Box<WithSpan<Self>>,
        next: Box<WithSpan<Self>>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchCase {
    pub ty: WithSpan<Type>,
    pub expr: WithSpan<Expr>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MapElem {
    pub key: WithSpan<Expr>,
    pub value: WithSpan<Expr>,
}
