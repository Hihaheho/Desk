use uuid::Uuid;

use crate::{span::Spanned, ty::Type};

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    String(String),
    Int(i64),
    Rational(i64, i64),
    Float(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Let {
        ty: Spanned<Type>,
        definition: Box<Spanned<Self>>,
        expression: Box<Spanned<Self>>,
    },
    Perform {
        input: Box<Spanned<Self>>,
        output: Spanned<Type>,
    },
    Handle {
        input: Spanned<Type>,
        output: Spanned<Type>,
        handler: Box<Spanned<Self>>,
        expr: Box<Spanned<Self>>,
    },
    Apply {
        function: Spanned<Type>,
        arguments: Vec<Spanned<Self>>,
    },
    Product(Vec<Spanned<Self>>),
    Match {
        of: Box<Spanned<Self>>,
        cases: Vec<MatchCase>,
    },
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
    Include(String),
    Import {
        ty: Spanned<Type>,
        uuid: Option<Uuid>,
    },
    Export {
        ty: Spanned<Type>,
    },
    Attribute {
        attr: Box<Spanned<Self>>,
        expr: Box<Spanned<Self>>,
    },
    Brand {
        brands: Vec<String>,
        expr: Box<Spanned<Self>>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct MatchCase {
    pub ty: Spanned<Type>,
    pub expr: Spanned<Expr>,
}
