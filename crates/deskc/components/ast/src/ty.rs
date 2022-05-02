use crate::{expr::Expr, span::Spanned};

#[derive(Clone, Debug, PartialEq)]
pub struct Effect {
    pub input: Spanned<Type>,
    pub output: Spanned<Type>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Brand {
        brand: String,
        item: Box<Spanned<Type>>,
    },
    Number,
    String,
    Trait(Vec<Spanned<Self>>),
    Effectful {
        ty: Box<Spanned<Self>>,
        effects: Spanned<EffectExpr>,
    },
    Infer,
    This,
    Alias(String),
    Product(Vec<Spanned<Self>>),
    Sum(Vec<Spanned<Self>>),
    Function {
        parameters: Vec<Spanned<Self>>,
        body: Box<Spanned<Self>>,
    },
    Array(Box<Spanned<Self>>),
    Set(Box<Spanned<Self>>),
    Let {
        variable: String,
        body: Box<Spanned<Self>>,
    },
    Variable(String),
    BoundedVariable {
        bound: Box<Spanned<Self>>,
        identifier: String,
    },
    Attribute {
        attr: Box<Spanned<Expr>>,
        ty: Box<Spanned<Self>>,
    },
    Comment {
        position: CommentPosition,
        text: String,
        item: Box<Spanned<Self>>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum CommentPosition {
    Prefix,
    Suffix,
}

#[derive(Clone, Debug, PartialEq)]
pub enum EffectExpr {
    Effects(Vec<Spanned<Effect>>),
    Add(Vec<Spanned<EffectExpr>>),
    Sub {
        minuend: Box<Spanned<EffectExpr>>,
        subtrahend: Box<Spanned<EffectExpr>>,
    },
    Apply {
        function: Box<Spanned<Type>>,
        arguments: Vec<Spanned<Type>>,
    },
}
