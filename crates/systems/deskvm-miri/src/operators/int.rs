use crate::value::{OperatorOutput, Value};

use super::helpers::lr;

pub fn add(value: &Value) -> OperatorOutput {
    let (l, r) = int_lr(value);
    OperatorOutput::Return(Value::Int(l + r))
}

pub fn sub(value: &Value) -> OperatorOutput {
    let (l, r) = int_lr(value);
    OperatorOutput::Return(Value::Int(l - r))
}

pub fn mul(value: &Value) -> OperatorOutput {
    let (l, r) = int_lr(value);
    OperatorOutput::Return(Value::Int(l * r))
}

pub fn div(value: &Value) -> OperatorOutput {
    let (l, r) = int_lr(value);
    if r == 0 {
        OperatorOutput::Perform {
            effect: macros::effect!(r#"@"division by zero" 'integer ~> @"quot" 'integer"#),
            input: Value::Int(l),
        }
    } else {
        OperatorOutput::Return(Value::Int(l / r))
    }
}

pub fn int_lr(value: &Value) -> (i64, i64) {
    let (l, r) = lr(value);
    let Value::Int(l) = l else { panic!("left operand of integer operator not an integer")};
    let Value::Int(r) = r else { panic!("right operand of integer operator not an integer")};
    (*l, *r)
}
