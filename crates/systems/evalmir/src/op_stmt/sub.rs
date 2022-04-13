use crate::value::Value;

pub(crate) fn calc(lhs: &Value, rhs: &Value) -> Value {
    match (lhs, rhs) {
        (Value::Int(int), Value::Int(int2)) => Value::Int(int - int2),
        (Value::Float(float), Value::Float(float2)) => Value::Float(float - float2),
        (Value::Rational(a, b), Value::Rational(a2, b2)) => {
            // TODO: use GCD and LCM
            let a = a * b2 - a2 * b;
            let b = b * b2;
            if a % b == 0 {
                Value::Int(a / b)
            } else {
                Value::Rational(a, b)
            }
        }
        (Value::Int(int), Value::Float(float)) => Value::Float(*int as f64 - float),
        (Value::Float(float), Value::Int(int)) => Value::Float(float - *int as f64),
        (Value::Int(int), Value::Rational(a, b)) => Value::Rational(int * b - a, *b),
        (Value::Rational(a, b), Value::Int(int)) => Value::Rational(a - int * b, *b),
        (Value::Float(float), Value::Rational(a, b)) => Value::Float(float - *a as f64 / *b as f64),
        (Value::Rational(a, b), Value::Float(float)) => Value::Float(*a as f64 - float / *b as f64),
        _ => panic!("adds not numbers"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn rational() {
        assert_eq!(calc(&Value::Int(3), &Value::Int(1)), Value::Int(2));
        assert_eq!(
            calc(&Value::Rational(2, 3), &Value::Rational(1, 2)),
            Value::Rational(1, 6)
        );
        assert_eq!(
            calc(&Value::Rational(4, 3), &Value::Rational(1, 3)),
            Value::Int(1)
        );
        assert_eq!(
            calc(&Value::Int(3), &Value::Rational(1, 2)),
            Value::Rational(5, 2)
        );
        assert_eq!(
            calc(&Value::Rational(1, 2), &Value::Int(3)),
            Value::Rational(-5, 2)
        );
        assert_eq!(
            calc(&Value::Float(3.0), &Value::Rational(1, 2)),
            Value::Float(2.5)
        );
    }
}
