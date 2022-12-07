use std::collections::HashMap;

use dson::Dson;
use ids::NodeId;
use serde::{Deserialize, Serialize};

pub type Id = usize;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Effect {
    pub input: Type,
    pub output: Type,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Type {
    Real,
    Rational,
    Integer,
    String,
    Product(Vec<Self>),
    Sum(Vec<Self>),
    Function {
        parameter: Box<Self>,
        body: Box<Self>,
    },
    Vector(Box<Self>),
    Map {
        key: Box<Self>,
        value: Box<Self>,
    },
    Variable(String),
    ForAll {
        variable: String,
        bound: Option<Box<Self>>,
        body: Box<Self>,
    },
    Effectful {
        ty: Box<Self>,
        effects: EffectExpr,
    },
    Brand {
        brand: Dson,
        item: Box<Self>,
    },
    Label {
        label: Dson,
        item: Box<Self>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EffectExpr {
    Effects(Vec<Effect>),
    Add(Vec<EffectExpr>),
    Sub {
        minuend: Box<EffectExpr>,
        subtrahend: Box<EffectExpr>,
    },
    Apply {
        function: Box<Type>,
        arguments: Vec<Type>,
    },
}

impl Type {
    pub fn unit() -> Self {
        Type::Product(vec![])
    }
    pub fn label(label: Dson, item: Self) -> Self {
        Type::Label {
            label: label,
            item: Box::new(item),
        }
    }
    pub fn product(mut types: Vec<Self>) -> Self {
        // Sort is required to make the type comparable.
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
    pub fn function(parameter: Self, body: Self) -> Self {
        Type::Function {
            parameter: Box::new(parameter),
            body: Box::new(body),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Types {
    pub types: HashMap<NodeId, Type>,
}

impl Types {
    pub fn get(&self, id: &NodeId) -> Option<&Type> {
        self.types.get(id)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
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
