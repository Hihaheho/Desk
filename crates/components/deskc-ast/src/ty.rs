use dson::Dson;

use crate::span::WithSpan;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Effect {
    pub input: WithSpan<Type>,
    pub output: WithSpan<Type>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Brand {
        brand: Dson,
        item: Box<WithSpan<Type>>,
    },
    Number,
    String,
    Trait(Vec<WithSpan<Function>>),
    Effectful {
        ty: Box<WithSpan<Self>>,
        effects: WithSpan<EffectExpr>,
    },
    Infer,
    This,
    Product(Vec<WithSpan<Self>>),
    Sum(Vec<WithSpan<Self>>),
    Function(Box<Function>),
    Vector(Box<WithSpan<Self>>),
    Map {
        key: Box<WithSpan<Self>>,
        value: Box<WithSpan<Self>>,
    },
    Let {
        variable: String,
        definition: Box<WithSpan<Self>>,
        body: Box<WithSpan<Self>>,
    },
    Variable(String),
    BoundedVariable {
        bound: Box<WithSpan<Self>>,
        identifier: String,
    },
    Attributed {
        attr: Dson,
        ty: Box<WithSpan<Self>>,
    },
    Comment {
        text: String,
        item: Box<WithSpan<Self>>,
    },
    Forall {
        variable: String,
        bound: Option<Box<WithSpan<Self>>>,
        body: Box<WithSpan<Self>>,
    },
    Exists {
        variable: String,
        bound: Option<Box<WithSpan<Self>>>,
        body: Box<WithSpan<Self>>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function {
    pub parameter: WithSpan<Type>,
    pub body: WithSpan<Type>,
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
