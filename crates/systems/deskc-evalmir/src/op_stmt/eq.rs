use crate::value::Value;

pub(crate) fn calc(lhs: &Value, rhs: &Value) -> Value {
    let eq = |pred| {
        if pred {
            Value::Variant {
                id: 0,
                value: Box::new(Value::Tuple(vec![])),
            }
        } else {
            Value::Variant {
                id: 1,
                value: Box::new(Value::Tuple(vec![])),
            }
        }
    };
    match (lhs, rhs) {
        (x, y) if x == y => eq(true),
        (Value::Int(int), Value::Int(int2)) => eq(int == int2),
        (Value::Float(float), Value::Float(float2)) => eq(float == float2),
        (Value::Rational(a, b), Value::Rational(a2, b2)) => eq(a * b2 == a2 * b),
        (Value::Int(int), Value::Float(float)) | (Value::Float(float), Value::Int(int)) => {
            eq(*int as f64 == *float)
        }
        (Value::Int(int), Value::Rational(a, b)) | (Value::Rational(a, b), Value::Int(int)) => {
            eq(*a as f64 / *b as f64 == *int as f64)
        }
        (Value::Float(float), Value::Rational(a, b))
        | (Value::Rational(a, b), Value::Float(float)) => eq(*float == *a as f64 / *b as f64),
        _ => panic!("adds not numbers"),
    }
}
