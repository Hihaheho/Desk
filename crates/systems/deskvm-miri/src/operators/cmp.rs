use std::cmp::Ordering;

use crate::value::{OperatorOutput, Value};

use super::helpers::lr;

pub fn real_eq(value: &Value) -> OperatorOutput {
    let value = match compare(value) {
        Ordering::Less | Ordering::Greater => Value::Variant {
            ty: macros::ty!(r#"@"unequal" *<>"#),
            value: Box::new(Value::Unit),
        },
        Ordering::Equal => Value::Variant {
            ty: macros::ty!(r#"@"equal" *<>"#),
            value: Box::new(Value::Unit),
        },
    };
    OperatorOutput::Return(value)
}

pub fn real_cmp(value: &Value) -> OperatorOutput {
    let value = match compare(value) {
        Ordering::Less => Value::Variant {
            ty: macros::ty!(r#"@"less" *<>"#),
            value: Box::new(Value::Unit),
        },
        Ordering::Greater => Value::Variant {
            ty: macros::ty!(r#"@"greater" *<>"#),
            value: Box::new(Value::Unit),
        },
        Ordering::Equal => Value::Variant {
            ty: macros::ty!(r#"@"equal" *<>"#),
            value: Box::new(Value::Unit),
        },
    };
    OperatorOutput::Return(value)
}

fn compare(value: &Value) -> Ordering {
    let (l, r) = lr(value);

    match (l, r) {
        (Value::Int(l), Value::Int(r)) => ord(l, r),
        (Value::Rational(a, b), Value::Rational(a2, b2)) => ord(a * *b2 as i64, *b as i64 * a2),
        (Value::Rational(a, b), Value::Int(i)) => ord(*a, *b as i64 * i),
        (Value::Int(i), Value::Rational(a, b)) => ord(*b as i64 * i, *a),
        (Value::Real(a), Value::Real(b)) => ord(a, b),
        (Value::Real(r), Value::Int(i)) => ord(*r, *i as f64),
        (Value::Int(i), Value::Real(r)) => ord(*i as f64, *r),
        (Value::Real(r), Value::Rational(a, b)) => ord(*r, *a as f64 / *b as f64),
        (Value::Rational(a, b), Value::Real(r)) => ord(*a as f64 / *b as f64, *r),
        _ => panic!("Expected numbers"),
    }
}

fn ord<T: PartialOrd>(l: T, r: T) -> Ordering {
    if l < r {
        Ordering::Less
    } else if l > r {
        Ordering::Greater
    } else {
        // This is safe because we don't have NaNs in Desk-lang
        Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_int() {
        let value = Value::Product(
            [
                (macros::ty!(r#"@"l" 'integer"#), Value::Int(1)),
                (macros::ty!(r#"@"r" 'integer"#), Value::Int(2)),
            ]
            .into_iter()
            .collect(),
        );
        assert_eq!(compare(&value), Ordering::Less);
        let value = Value::Product(
            [
                (macros::ty!(r#"@"l" 'integer"#), Value::Int(2)),
                (macros::ty!(r#"@"r" 'integer"#), Value::Int(1)),
            ]
            .into_iter()
            .collect(),
        );
        assert_eq!(compare(&value), Ordering::Greater);
        let value = Value::Product(
            [
                (macros::ty!(r#"@"l" 'integer"#), Value::Int(2)),
                (macros::ty!(r#"@"r" 'integer"#), Value::Int(2)),
            ]
            .into_iter()
            .collect(),
        );
        assert_eq!(compare(&value), Ordering::Equal);
    }

    #[test]
    fn test_rational_rational() {
        let value = Value::Product(
            [
                (macros::ty!(r#"@"l" 'rational"#), Value::Rational(1, 2)),
                (macros::ty!(r#"@"r" 'rational"#), Value::Rational(1, 3)),
            ]
            .into_iter()
            .collect(),
        );
        assert_eq!(compare(&value), Ordering::Greater);
        let value = Value::Product(
            [
                (macros::ty!(r#"@"l" 'rational"#), Value::Rational(1, 3)),
                (macros::ty!(r#"@"r" 'rational"#), Value::Rational(1, 2)),
            ]
            .into_iter()
            .collect(),
        );
        assert_eq!(compare(&value), Ordering::Less);
        let value = Value::Product(
            [
                (macros::ty!(r#"@"l" 'rational"#), Value::Rational(1, 2)),
                (macros::ty!(r#"@"r" 'rational"#), Value::Rational(2, 4)),
            ]
            .into_iter()
            .collect(),
        );
        assert_eq!(compare(&value), Ordering::Equal);
    }

    #[test]
    fn test_real_real() {
        let value = Value::Product(
            [
                (macros::ty!(r#"@"l" 'real"#), Value::Real(1.0)),
                (macros::ty!(r#"@"r" 'real"#), Value::Real(2.0)),
            ]
            .into_iter()
            .collect(),
        );
        assert_eq!(compare(&value), Ordering::Less);
        let value = Value::Product(
            [
                (macros::ty!(r#"@"l" 'real"#), Value::Real(2.0)),
                (macros::ty!(r#"@"r" 'real"#), Value::Real(1.0)),
            ]
            .into_iter()
            .collect(),
        );
        assert_eq!(compare(&value), Ordering::Greater);
        let value = Value::Product(
            [
                (macros::ty!(r#"@"l" 'real"#), Value::Real(2.0)),
                (macros::ty!(r#"@"r" 'real"#), Value::Real(2.0)),
            ]
            .into_iter()
            .collect(),
        );
        assert_eq!(compare(&value), Ordering::Equal);
    }

    #[test]
    fn test_real_integer() {
        let value = Value::Product(
            [
                (macros::ty!(r#"@"l" 'real"#), Value::Real(1.0)),
                (macros::ty!(r#"@"r" 'integer"#), Value::Int(2)),
            ]
            .into_iter()
            .collect(),
        );
        assert_eq!(compare(&value), Ordering::Less);
        let value = Value::Product(
            [
                (macros::ty!(r#"@"l" 'real"#), Value::Real(2.0)),
                (macros::ty!(r#"@"r" 'integer"#), Value::Int(1)),
            ]
            .into_iter()
            .collect(),
        );
        assert_eq!(compare(&value), Ordering::Greater);
        let value = Value::Product(
            [
                (macros::ty!(r#"@"l" 'real"#), Value::Real(2.0)),
                (macros::ty!(r#"@"r" 'integer"#), Value::Int(2)),
            ]
            .into_iter()
            .collect(),
        );
        assert_eq!(compare(&value), Ordering::Equal);
    }

    #[test]
    fn test_integer_real() {
        let value = Value::Product(
            [
                (macros::ty!(r#"@"l" 'integer"#), Value::Int(1)),
                (macros::ty!(r#"@"r" 'real"#), Value::Real(2.0)),
            ]
            .into_iter()
            .collect(),
        );
        assert_eq!(compare(&value), Ordering::Less);
        let value = Value::Product(
            [
                (macros::ty!(r#"@"l" 'integer"#), Value::Int(2)),
                (macros::ty!(r#"@"r" 'real"#), Value::Real(1.0)),
            ]
            .into_iter()
            .collect(),
        );
        assert_eq!(compare(&value), Ordering::Greater);
        let value = Value::Product(
            [
                (macros::ty!(r#"@"l" 'integer"#), Value::Int(2)),
                (macros::ty!(r#"@"r" 'real"#), Value::Real(2.0)),
            ]
            .into_iter()
            .collect(),
        );
        assert_eq!(compare(&value), Ordering::Equal);
    }

    #[test]
    fn test_integer_rational() {
        let value = Value::Product(
            [
                (macros::ty!(r#"@"l" 'integer"#), Value::Int(1)),
                (macros::ty!(r#"@"r" 'rational"#), Value::Rational(3, 2)),
            ]
            .into_iter()
            .collect(),
        );
        assert_eq!(compare(&value), Ordering::Less);
        let value = Value::Product(
            [
                (macros::ty!(r#"@"l" 'integer"#), Value::Int(2)),
                (macros::ty!(r#"@"r" 'rational"#), Value::Rational(2, 3)),
            ]
            .into_iter()
            .collect(),
        );
        assert_eq!(compare(&value), Ordering::Greater);
        let value = Value::Product(
            [
                (macros::ty!(r#"@"l" 'integer"#), Value::Int(2)),
                (macros::ty!(r#"@"r" 'rational"#), Value::Rational(4, 2)),
            ]
            .into_iter()
            .collect(),
        );
        assert_eq!(compare(&value), Ordering::Equal);
    }

    #[test]
    fn test_rational_integer() {
        let value = Value::Product(
            [
                (macros::ty!(r#"@"l" 'rational"#), Value::Rational(3, 2)),
                (macros::ty!(r#"@"r" 'integer"#), Value::Int(1)),
            ]
            .into_iter()
            .collect(),
        );
        assert_eq!(compare(&value), Ordering::Greater);
        let value = Value::Product(
            [
                (macros::ty!(r#"@"l" 'rational"#), Value::Rational(3, 2)),
                (macros::ty!(r#"@"r" 'integer"#), Value::Int(2)),
            ]
            .into_iter()
            .collect(),
        );
        assert_eq!(compare(&value), Ordering::Less);
        let value = Value::Product(
            [
                (macros::ty!(r#"@"l" 'rational"#), Value::Rational(4, 2)),
                (macros::ty!(r#"@"r" 'integer"#), Value::Int(2)),
            ]
            .into_iter()
            .collect(),
        );
        assert_eq!(compare(&value), Ordering::Equal);
    }

    #[test]
    fn test_real_rational() {
        let value = Value::Product(
            [
                (macros::ty!(r#"@"l" 'real"#), Value::Real(1.0)),
                (macros::ty!(r#"@"r" 'rational"#), Value::Rational(3, 2)),
            ]
            .into_iter()
            .collect(),
        );
        assert_eq!(compare(&value), Ordering::Less);
        let value = Value::Product(
            [
                (macros::ty!(r#"@"l" 'real"#), Value::Real(2.0)),
                (macros::ty!(r#"@"r" 'rational"#), Value::Rational(3, 2)),
            ]
            .into_iter()
            .collect(),
        );
        assert_eq!(compare(&value), Ordering::Greater);
        let value = Value::Product(
            [
                (macros::ty!(r#"@"l" 'real"#), Value::Real(2.0)),
                (macros::ty!(r#"@"r" 'rational"#), Value::Rational(4, 2)),
            ]
            .into_iter()
            .collect(),
        );
        assert_eq!(compare(&value), Ordering::Equal);
    }

    #[test]
    fn test_rational_real() {
        let value = Value::Product(
            [
                (macros::ty!(r#"@"l" 'rational"#), Value::Rational(3, 2)),
                (macros::ty!(r#"@"r" 'real"#), Value::Real(1.0)),
            ]
            .into_iter()
            .collect(),
        );
        assert_eq!(compare(&value), Ordering::Greater);
        let value = Value::Product(
            [
                (macros::ty!(r#"@"l" 'rational"#), Value::Rational(3, 2)),
                (macros::ty!(r#"@"r" 'real"#), Value::Real(2.0)),
            ]
            .into_iter()
            .collect(),
        );
        assert_eq!(compare(&value), Ordering::Less);
        let value = Value::Product(
            [
                (macros::ty!(r#"@"l" 'rational"#), Value::Rational(4, 2)),
                (macros::ty!(r#"@"r" 'real"#), Value::Real(2.0)),
            ]
            .into_iter()
            .collect(),
        );
        assert_eq!(compare(&value), Ordering::Equal);
    }
}
