use uuid::Uuid;

use crate::{r#type::Type, span::Spanned};

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    String(String),
    Int(i64),
    Rational(i64, i64),
    Float(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Handler {
    pub ty: Spanned<Type>,
    pub expr: Spanned<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Let {
        definition: Box<Spanned<Self>>,
        expression: Box<Spanned<Self>>,
    },
    Perform {
        effect: Box<Spanned<Self>>,
    },
    Effectful {
        class: Spanned<Type>,
        expr: Box<Spanned<Self>>,
        handlers: Vec<Handler>,
    },
    Call {
        function: Spanned<Type>,
        arguments: Vec<Spanned<Self>>,
    },
    Product(Vec<Spanned<Self>>),
    Typed {
        ty: Spanned<Type>,
        expr: Box<Spanned<Self>>,
    },
    Hole,
    Function {
        parameters: Vec<Spanned<Type>>,
        body: Box<Spanned<Self>>,
    },
    Array(Vec<Spanned<Self>>),
    Set(Vec<Spanned<Self>>),
    Module(Box<Spanned<Self>>),
    Import {
        ty: Spanned<Type>,
        uuid: Option<Uuid>,
    },
    Export {
        ty: Spanned<Type>,
    },
}
