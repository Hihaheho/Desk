use std::collections::HashMap;

use dprocess::value::Number;
use mir::mir::ControlFlowGraphId;
use types::{Effect, Type};

use crate::{eval_cfg::Handler, operators::Operator};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    // empty product
    Unit,
    String(String),
    Int(i64),
    Real(f64),
    // b must be unsigned to avoid ambiguity.
    Rational(i64, u64),
    Product(HashMap<Type, Value>),
    Variant { ty: Type, value: Box<Value> },
    Vector(Vec<Self>),
    FnRef(FnRef),
    TraitObject { ty: Type, value: Box<Value> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum FnRef {
    Link(Type),
    Closure(Closure),
    Recursion,
    Operator(Operator),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Closure {
    pub mir: ControlFlowGraphId,
    pub captured: HashMap<Type, Value>,
    pub handlers: HashMap<Effect, Handler>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OperatorOutput {
    Return(Value),
    Perform { effect: Effect, input: Value },
}

impl From<Value> for dprocess::value::Value {
    fn from(value: Value) -> Self {
        match value {
            Value::Unit => dprocess::value::Value::Unit,
            Value::String(string) => dprocess::value::Value::String(string),
            Value::Int(int) => dprocess::value::Value::Number(Number::Integer(int)),
            Value::Real(float) => dprocess::value::Value::Number(Number::Real(float)),
            Value::Rational(a, b) => dprocess::value::Value::Number(Number::Rational(a, b)),
            Value::Product(values) => dprocess::value::Value::Product(
                values
                    .into_iter()
                    .map(|(ty, value)| (ty, value.into()))
                    .collect(),
            ),
            Value::Variant { ty, value } => dprocess::value::Value::Variant {
                ty,
                value: Box::new((*value).into()),
            },
            Value::Vector(values) => {
                dprocess::value::Value::Vector(values.into_iter().map(Into::into).collect())
            }
            Value::FnRef(_) => panic!(),
            Value::TraitObject { ty, value } => dprocess::value::Value::TraitObject {
                ty,
                value: Box::new((*value).into()),
            },
        }
    }
}

impl From<dprocess::value::Value> for Value {
    fn from(value: dprocess::value::Value) -> Self {
        match value {
            dprocess::value::Value::Unit => Value::Unit,
            dprocess::value::Value::Number(number) => match number {
                Number::Integer(int) => Value::Int(int),
                Number::Real(float) => Value::Real(float),
                Number::Rational(a, b) => Value::Rational(a, b),
            },
            dprocess::value::Value::String(string) => Value::String(string),
            dprocess::value::Value::Product(values) => Value::Product(
                values
                    .into_iter()
                    .map(|(ty, value)| (ty, value.into()))
                    .collect(),
            ),
            dprocess::value::Value::Variant { ty, value } => Value::Variant {
                ty,
                value: Box::new((*value).into()),
            },
            dprocess::value::Value::Vector(values) => {
                Value::Vector(values.into_iter().map(Into::into).collect())
            }
            dprocess::value::Value::TraitObject { ty, value } => Value::TraitObject {
                ty,
                value: Box::new((*value).into()),
            },
        }
    }
}
