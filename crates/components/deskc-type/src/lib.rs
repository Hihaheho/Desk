pub mod conclusion;

use dson::Dson;
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
    Function(Box<Function>),
    Trait(Vec<Function>),
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

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Function {
    pub parameter: Type,
    pub body: Type,
}

impl Type {
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
        Type::Function(Box::new(Function { parameter, body }))
    }

    pub fn parameters(&self) -> ParametersIter<'_> {
        ParametersIter(self)
    }
}

pub struct ParametersIter<'a>(&'a Type);

impl<'a> Iterator for ParametersIter<'a> {
    type Item = &'a Type;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            Type::Function(f) => {
                self.0 = &f.body;
                Some(&f.parameter)
            }
            Type::ForAll { body, .. } => {
                self.0 = body;
                self.next()
            }
            _ => None,
        }
    }
}
