use std::collections::HashMap;

use thir::BuiltinOp;
use types::{Effect, Type};

pub(crate) fn find_builtin(ty: &Type) -> Option<(BuiltinOp, usize)> {
    let map: HashMap<_, _> = [
        (
            Type::function(
                vec![Type::Number, Type::Number],
                labeled("sum", Type::Number),
            ),
            (BuiltinOp::Add, 2),
        ),
        (
            Type::function(
                vec![
                    labeled("minuend", Type::Number),
                    labeled("subtrahend", Type::Number),
                ],
                Type::Number,
            ),
            (BuiltinOp::Sub, 2),
        ),
        (
            Type::function(
                vec![Type::Number, Type::Number],
                labeled("product", Type::Number),
            ),
            (BuiltinOp::Mul, 2),
        ),
        (
            Type::function(
                vec![
                    labeled("dividend", Type::Number),
                    labeled("divisor", Type::Number),
                ],
                Type::Effectful {
                    ty: Box::new(Type::Number),
                    effects: vec![Effect {
                        input: labeled("division by zero", Type::Number),
                        output: Type::Number,
                    }],
                },
            ),
            (BuiltinOp::Div, 2),
        ),
        (
            Type::function(
                vec![
                    labeled("dividend", Type::Number),
                    labeled("divisor", Type::Number),
                ],
                Type::Effectful {
                    ty: Box::new(labeled("quotient", Type::Number)),
                    effects: vec![Effect {
                        input: labeled("division by zero", Type::Number),
                        output: Type::Number,
                    }],
                },
            ),
            (BuiltinOp::Div, 2),
        ),
        (
            Type::function(
                vec![
                    labeled("dividend", Type::Number),
                    labeled("divisor", Type::Number),
                ],
                Type::Effectful {
                    ty: Box::new(labeled("remainder", Type::Number)),
                    effects: vec![Effect {
                        input: labeled("division by zero", Type::Number),
                        output: Type::Number,
                    }],
                },
            ),
            (BuiltinOp::Rem, 2),
        ),
    ]
    .into_iter()
    .collect();
    map.get(&ty).cloned()
}

fn labeled(label: &str, item: Type) -> Type {
    Type::Label {
        label: label.into(),
        item: Box::new(item),
    }
}
