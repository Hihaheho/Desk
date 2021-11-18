use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Handler {
    pub input: Type,
    pub output: Type,
}

pub type Id = usize;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Effect {
    pub input: Type,
    pub output: Type,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Number,
    String,
    Product(Vec<Self>),
    Sum(Vec<Self>),
    Function {
        parameter: Box<Self>,
        body: Box<Self>,
    },
    Array(Box<Self>),
    Set(Box<Self>),
    Variable(Id),
    ForAll {
        variable: Id,
        body: Box<Self>,
    },
    Existential(Id),
    Effectful {
        ty: Box<Self>,
        effects: Vec<Effect>,
    },
}

pub struct ExprTypes {
    pub types: HashMap<Id, Type>,
}

mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
