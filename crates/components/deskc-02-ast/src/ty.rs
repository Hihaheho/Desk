use crate::{expr::Expr, span::WithSpan};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Effect {
    pub input: WithSpan<Type>,
    pub output: WithSpan<Type>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Brand {
        brand: String,
        item: Box<WithSpan<Type>>,
    },
    Number,
    String,
    Trait(Vec<WithSpan<Self>>),
    Effectful {
        ty: Box<WithSpan<Self>>,
        effects: WithSpan<EffectExpr>,
    },
    Infer,
    This,
    Product(Vec<WithSpan<Self>>),
    Sum(Vec<WithSpan<Self>>),
    Function {
        parameters: Vec<WithSpan<Self>>,
        body: Box<WithSpan<Self>>,
    },
    Vector(Box<WithSpan<Self>>),
    Set(Box<WithSpan<Self>>),
    Let {
        variable: String,
        body: Box<WithSpan<Self>>,
    },
    Variable(String),
    BoundedVariable {
        bound: Box<WithSpan<Self>>,
        identifier: String,
    },
    Attribute {
        attr: Box<WithSpan<Expr>>,
        ty: Box<WithSpan<Self>>,
    },
    Comment {
        position: CommentPosition,
        text: String,
        item: Box<WithSpan<Self>>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommentPosition {
    Prefix,
    Suffix,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EffectExpr {
    Effects(Vec<WithSpan<Effect>>),
    Add(Vec<WithSpan<EffectExpr>>),
    Sub {
        minuend: Box<WithSpan<EffectExpr>>,
        subtrahend: Box<WithSpan<EffectExpr>>,
    },
    Apply {
        function: Box<WithSpan<Type>>,
        arguments: Vec<WithSpan<Type>>,
    },
}
