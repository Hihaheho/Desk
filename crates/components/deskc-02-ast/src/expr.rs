pub use ids::LinkName;
use uuid::Uuid;

use crate::{
    span::WithSpan,
    ty::{CommentPosition, Type},
};

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    String(String),
    Integer(i64),
    Rational(i64, i64),
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
    Let {
        ty: WithSpan<Type>,
        definition: Box<WithSpan<Self>>,
        body: Box<WithSpan<Self>>,
    },
    Perform {
        input: Box<WithSpan<Self>>,
        output: WithSpan<Type>,
    },
    Continue {
        input: Box<WithSpan<Self>>,
        output: Option<WithSpan<Type>>,
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
        parameters: Vec<WithSpan<Type>>,
        body: Box<WithSpan<Self>>,
    },
    Vector(Vec<WithSpan<Self>>),
    Set(Vec<WithSpan<Self>>),
    Import {
        ty: WithSpan<Type>,
        uuid: Option<Uuid>,
    },
    Export {
        ty: WithSpan<Type>,
    },
    Attribute {
        attr: Box<WithSpan<Self>>,
        item: Box<WithSpan<Self>>,
    },
    Brand {
        brands: Vec<String>,
        item: Box<WithSpan<Self>>,
    },
    Label {
        label: String,
        item: Box<WithSpan<Self>>,
    },
    NewType {
        ident: String,
        ty: WithSpan<Type>,
        expr: Box<WithSpan<Self>>,
    },
    Comment {
        position: CommentPosition,
        text: String,
        item: Box<WithSpan<Self>>,
    },
    Card {
        uuid: Uuid,
        item: Box<WithSpan<Self>>,
        next: Option<Box<WithSpan<Self>>>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchCase {
    pub ty: WithSpan<Type>,
    pub expr: WithSpan<Expr>,
}
