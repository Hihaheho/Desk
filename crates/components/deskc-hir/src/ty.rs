use dson::Dson;

use crate::meta::WithMeta;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Handler {
    pub input: WithMeta<Type>,
    pub output: WithMeta<Type>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Effect {
    pub input: WithMeta<Type>,
    pub output: WithMeta<Type>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Type {
    Real,
    Rational,
    Integer,
    String,
    Effectful {
        ty: Box<WithMeta<Self>>,
        effects: WithMeta<EffectExpr>,
    },
    Infer,
    Product(Vec<WithMeta<Self>>),
    Sum(Vec<WithMeta<Self>>),
    Function(Box<Function>),
    Vector(Box<WithMeta<Self>>),
    Map {
        key: Box<WithMeta<Self>>,
        value: Box<WithMeta<Self>>,
    },
    Let {
        variable: String,
        // definition: Box<WithMeta<Self>>,
        body: Box<WithMeta<Self>>,
        definition: Box<WithMeta<Type>>,
    },
    Variable(String),
    Brand {
        brand: String,
        item: Box<WithMeta<Self>>,
    },
    // just label brand
    Label {
        label: String,
        item: Box<WithMeta<Self>>,
    },
    Forall {
        variable: String,
        bound: Option<Box<WithMeta<Self>>>,
        body: Box<WithMeta<Self>>,
    },
    Exists {
        variable: String,
        bound: Option<Box<WithMeta<Self>>>,
        body: Box<WithMeta<Self>>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Function {
    pub parameter: WithMeta<Type>,
    pub body: WithMeta<Type>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum EffectExpr {
    Effects(Vec<WithMeta<Effect>>),
    Add(Vec<WithMeta<EffectExpr>>),
    Sub {
        minuend: Box<WithMeta<EffectExpr>>,
        subtrahend: Box<WithMeta<EffectExpr>>,
    },
    Apply {
        function: Box<WithMeta<Type>>,
        arguments: Vec<WithMeta<Type>>,
    },
}
