use crate::meta::WithMeta;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Handler {
    pub input: WithMeta<Type>,
    pub output: WithMeta<Type>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Number,
    String,
    Trait(Vec<WithMeta<Self>>),
    // Handlers do not need to be spanned because it has not leading token.
    Class(Vec<Handler>),
    Effectful {
        class: Box<WithMeta<Self>>,
        ty: Box<WithMeta<Self>>,
        handlers: Vec<Handler>,
    },
    Effect {
        class: Box<WithMeta<Self>>,
        handler: Box<Handler>,
    },
    Infer,
    This,
    Alias(String),
    Product(Vec<WithMeta<Self>>),
    Sum(Vec<WithMeta<Self>>),
    Function {
        parameters: Vec<WithMeta<Self>>,
        body: Box<WithMeta<Self>>,
    },
    Array(Box<WithMeta<Self>>),
    Set(Box<WithMeta<Self>>),
    Bound {
        bound: Box<WithMeta<Self>>,
        item: Box<WithMeta<Self>>,
    },
    Let {
        definition: Box<WithMeta<Self>>,
        body: Box<WithMeta<Self>>,
    },
    Identifier(String),
}
