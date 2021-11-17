use crate::span::Spanned;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Effect {
    pub input: Spanned<Type>,
    pub output: Spanned<Type>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Number,
    String,
    Trait(Vec<Spanned<Self>>),
    Effectful {
        ty: Box<Spanned<Self>>,
        effects: Vec<Effect>,
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
        definition: Box<Spanned<Self>>,
        body: Box<Spanned<Self>>,
    },
    Variable(String),
    BoundedVariable {
        bound: Box<Spanned<Self>>,
        identifier: String,
    },
}
