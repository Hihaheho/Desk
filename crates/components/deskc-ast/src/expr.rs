use dson::Dson;
use ids::CardId;
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
    pub effect: WithMeta<Effect>,
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
        body: Box<WithMeta<Self>>,
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
        expr: Box<WithMeta<Self>>,
        handlers: Vec<WithMeta<Handler>>,
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
    Hole,
    Function {
        parameter: WithMeta<Type>,
        body: Box<WithMeta<Self>>,
    },
    Vector(Vec<WithMeta<Self>>),
    Map(Vec<WithMeta<MapElem>>),
    Attributed {
        attr: Dson,
        item: Box<WithMeta<Self>>,
    },
    /// Declare the dson is a brand of the item.
    DeclareBrand {
        brand: Dson,
        item: Box<WithMeta<Self>>,
    },
    Label {
        label: Dson,
        item: Box<WithMeta<Self>>,
    },
    NewType {
        ident: String,
        ty: WithMeta<Type>,
        expr: Box<WithMeta<Self>>,
    },
    Card {
        id: CardId,
        item: Box<WithMeta<Self>>,
        next: Box<WithMeta<Self>>,
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
