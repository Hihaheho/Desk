pub mod visitor;

use dson::Dson;
pub use ids::LinkName;
use ids::NodeId;
use types::{Effect, Type};

pub type Id = usize;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Int(i64),
    Float(f64),
    // b must be unsigned to avoid ambiguity.
    Rational(i64, u64),
}

// Literal::Float should not be NaN
impl Eq for Literal {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchCase {
    pub ty: Type,
    pub expr: TypedHir,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedHir {
    pub id: NodeId,
    pub ty: Type,
    pub expr: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Handler {
    pub effect: Effect,
    pub handler: TypedHir,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Literal(Literal),
    Do {
        stmt: Box<TypedHir>,
        expr: Box<TypedHir>,
    },
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
    Apply {
        function: Type,
        link_name: LinkName,
        arguments: Vec<TypedHir>,
    },
    Product(Vec<TypedHir>),
    Function {
        parameter: Type,
        body: Box<TypedHir>,
    },
    Vector(Vec<TypedHir>),
    Map(Vec<MapElem>),
    Label {
        label: Dson,
        item: Box<TypedHir>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapElem {
    pub key: TypedHir,
    pub value: TypedHir,
}
