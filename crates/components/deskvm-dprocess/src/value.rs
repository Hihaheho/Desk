use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use types::Type;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// Sendable value between processes.
///
/// Closure is not sendable in the following reasons:
/// - DeskVM can have many types of interpreters, so it's hard to have a common closure format.
/// - Desk promotes data-oriented programming, so sending closures are not used as much as in other languages.
pub enum Value {
    /// It's actually a empty product.
    Unit,
    Number(Number),
    String(String),
    Product(HashMap<Type, Value>),
    Variant {
        ty: Type,
        value: Box<Value>,
    },
    Vector(Vec<Self>),
    TraitObject {
        ty: Type,
        value: Box<Value>,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Number {
    Integer(i64),
    Real(f64),
    // b must be unsigned to avoid ambiguity.
    Rational(i64, u64),
}

// A float of should not be NaN.
impl Eq for Number {}
