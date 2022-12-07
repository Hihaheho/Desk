use mir::stmt::Const;

use crate::value::Value;

pub(crate) fn eval(value: &Const) -> Value {
    match value {
        Const::Int(value) => Value::Int(*value),
        Const::Rational(a, b) => Value::Rational(*a, *b),
        Const::Real(value) => Value::Real(*value),
        Const::String(value) => Value::String(value.clone()),
    }
}
