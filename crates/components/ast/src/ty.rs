use crate::span::Spanned;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Handler {
    pub input: Spanned<Type>,
    pub output: Spanned<Type>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Number,
    String,
    Trait(Vec<Spanned<Self>>),
    // Handlers do not need to be spanned because it has not leading token.
    Class(Vec<Handler>),
    Effectful {
        class: Box<Spanned<Self>>,
        ty: Box<Spanned<Self>>,
        handlers: Vec<Handler>,
    },
    Effect {
        class: Box<Spanned<Self>>,
        handler: Box<Handler>,
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
    Bound {
        bound: Box<Spanned<Self>>,
        item: Box<Spanned<Self>>,
    },
    Let {
        definition: Box<Spanned<Self>>,
        body: Box<Spanned<Self>>,
    },
    Identifier(String),
}
