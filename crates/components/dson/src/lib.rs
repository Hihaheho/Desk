use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Real(pub f64);

impl PartialEq for Real {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

// Real must not be NaN or infinity.
impl Eq for Real {}

impl std::hash::Hash for Real {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_be_bytes().hash(state);
    }
}

impl PartialOrd for Real {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for Real {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Literal {
    String(String),
    Integer(i64),
    // b must be unsigned to avoid ambiguity.
    Rational(i64, u64),
    Real(Real),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Dson {
    Literal(Literal),
    Product(Vec<Self>),
    Vector(Vec<Self>),
    Map(Vec<MapElem>),
    Attributed { attr: Box<Self>, expr: Box<Self> },
    Labeled { label: String, expr: Box<Self> },
    Typed { ty: Type, expr: Box<Self> },
    Comment { text: String, expr: Box<Self> },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MapElem {
    pub key: Dson,
    pub value: Dson,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Type {
    Brand {
        brand: String,
        item: Box<Self>,
    },
    Real,
    Rational,
    Integer,
    String,
    Product(Vec<Self>),
    Sum(Vec<Self>),
    Vector(Box<Self>),
    Map {
        key: Box<Self>,
        value: Box<Self>,
    },
    Attributed {
        attr: Box<Dson>,
        ty: Box<Self>,
    },
    Comment {
        text: String,
        item: Box<Self>,
    },
    Let {
        variable: String,
        definition: Box<Self>,
        body: Box<Self>,
    },
    Variable(String),
}

impl From<&str> for Dson {
    fn from(s: &str) -> Self {
        Dson::Literal(Literal::String(s.to_string()))
    }
}

impl From<i64> for Dson {
    fn from(i: i64) -> Self {
        Dson::Literal(Literal::Integer(i))
    }
}

impl Display for Dson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dson::Literal(literal) => match literal {
                Literal::String(string) => write!(f, "{:?}", string),
                Literal::Integer(i) => write!(f, "{}", i),
                Literal::Rational(a, b) => write!(f, "{} / {}", a, b),
                Literal::Real(real) => write!(f, "{}", real.0),
            },
            Dson::Product(_) => todo!(),
            Dson::Vector(_) => todo!(),
            Dson::Map(_) => todo!(),
            Dson::Attributed { attr, expr } => todo!(),
            Dson::Labeled { label, expr } => todo!(),
            Dson::Typed { ty, expr } => todo!(),
            Dson::Comment { text, expr } => todo!(),
        }
    }
}
