use std::collections::HashMap;

pub type Id = usize;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Effect {
    pub input: Type,
    pub output: Type,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Type {
    Number,
    String,
    Product(Vec<Self>),
    Sum(Vec<Self>),
    Function {
        parameters: Vec<Self>,
        body: Box<Self>,
    },
    Array(Box<Self>),
    Set(Box<Self>),
    Variable(Id),
    ForAll {
        variable: Id,
        body: Box<Self>,
    },
    Effectful {
        ty: Box<Self>,
        effects: Vec<Effect>,
    },
    Brand {
        brand: String,
        item: Box<Self>,
    },
    Label {
        label: String,
        item: Box<Self>,
    },
}

impl Type {
    pub fn product(mut types: Vec<Self>) -> Self {
        types.sort();
        Type::Product(types)
    }
    pub fn sum(mut types: Vec<Self>) -> Self {
        types.sort();
        Type::Sum(types)
    }
    pub fn function(mut parameters: Vec<Self>, body: Self) -> Self {
        parameters.sort();
        Type::Function {
            parameters,
            body: Box::new(body),
        }
    }
    pub fn effectful(ty: Self, mut effects: Vec<Effect>) -> Self {
        effects.sort();
        Type::Effectful {
            ty: Box::new(ty),
            effects,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Types {
    pub types: HashMap<Id, Type>,
}

impl Types {
    pub fn get(&self, id: &Id) -> Option<&Type> {
        self.types.get(&id)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct IdGen {
    pub next: Id,
}

impl IdGen {
    pub fn next_id(&mut self) -> Id {
        let id = self.next;
        self.next += 1;
        id
    }
}
