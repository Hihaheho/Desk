use std::collections::HashMap;

pub type Id = usize;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Effect {
    pub input: Type,
    pub output: Type,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    pub fn unit() -> Self {
        Type::Product(vec![])
    }
    pub fn label(label: impl Into<String>, item: Self) -> Self {
        Type::Label {
            label: label.into(),
            item: Box::new(item),
        }
    }
    pub fn product(mut types: Vec<Self>) -> Self {
        types.sort();
        Type::Product(types)
    }
    pub fn sum(mut types: Vec<Self>) -> Self {
        types.sort();
        types.dedup();
        if types.len() == 1 {
            types.pop().unwrap()
        } else {
            Type::Sum(types)
        }
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
    pub next_id: Id,
}

impl IdGen {
    pub fn next_id(&mut self) -> Id {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}
