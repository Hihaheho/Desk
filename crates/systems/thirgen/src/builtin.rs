use std::{collections::HashMap, rc::Rc};

use hir::meta::WithMeta;
use thir::{BuiltinOp, Expr, Literal, TypedHir};
use types::{Effect, Type};

use crate::TypedHirGen;

#[derive(Clone)]
pub(crate) enum Builtin {
    Normal { op: BuiltinOp, params: usize },
    Custom(Rc<Box<dyn Fn(&TypedHirGen, &Vec<WithMeta<hir::expr::Expr>>) -> Expr>>),
}

pub(crate) fn find_builtin(ty: &Type) -> Option<Builtin> {
    let map: HashMap<_, _> = [
        (
            Type::function(
                vec![Type::Number, Type::Number],
                labeled("sum", Type::Number),
            ),
            Builtin::Normal {
                op: BuiltinOp::Add,
                params: 2,
            },
        ),
        (
            Type::function(
                vec![
                    labeled("minuend", Type::Number),
                    labeled("subtrahend", Type::Number),
                ],
                Type::Number,
            ),
            Builtin::Normal {
                op: BuiltinOp::Sub,
                params: 2,
            },
        ),
        (
            Type::function(
                vec![Type::Number, Type::Number],
                labeled("product", Type::Number),
            ),
            Builtin::Normal {
                op: BuiltinOp::Mul,
                params: 2,
            },
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
            Builtin::Custom(Rc::new(Box::new(|thirgen, args| {
                divide(thirgen, args, BuiltinOp::Div)
            }))),
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

fn divide(thirgen: &TypedHirGen, args: &Vec<WithMeta<hir::expr::Expr>>, op: BuiltinOp) -> Expr {
    assert_eq!(args.len(), 2, "args for div must be 2");
    let dividend = args[0].clone();
    let divisor = args[1].clone();
    Expr::Match {
        // zero check
        input: Box::new(TypedHir {
            id: thirgen.next_id(),
            ty: Type::sum(vec![
                Type::label("equal", Type::unit()),
                Type::label("unequal", Type::unit()),
            ]),
            expr: Expr::Op {
                op: BuiltinOp::Eq,
                operands: vec![
                    thirgen.gen(&divisor),
                    TypedHir {
                        id: thirgen.next_id(),
                        ty: Type::Number,
                        expr: Expr::Literal(Literal::Float(0.0)),
                    },
                ],
            },
        }),
        cases: vec![
            // If equal, perform division by zero
            thir::MatchCase {
                ty: Type::label("equal", Type::unit()),
                expr: TypedHir {
                    id: thirgen.next_id(),
                    ty: Type::Effectful {
                        ty: Box::new(Type::Number),
                        effects: vec![Effect {
                            input: Type::label("division by zero", Type::Number),
                            output: Type::Number,
                        }],
                    },
                    expr: Expr::Perform(Box::new(TypedHir {
                        id: thirgen.next_id(),
                        ty: Type::label("division by zero", Type::Number),
                        expr: Expr::Product(vec![]),
                    })),
                },
            },
            // If unequal, do division
            thir::MatchCase {
                ty: Type::label("unequal", Type::unit()),
                expr: TypedHir {
                    id: thirgen.next_id(),
                    ty: Type::label("quotient", Type::Number),
                    expr: Expr::Op {
                        op,
                        operands: vec![thirgen.gen(&dividend), thirgen.gen(&divisor)],
                    },
                },
            },
        ],
    }
}
