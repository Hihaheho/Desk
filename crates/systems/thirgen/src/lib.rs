mod builtin;

use std::cell::RefCell;

use hir::{
    expr::{Expr, Literal, MatchCase},
    meta::{Meta, WithMeta},
};
use thir::TypedHir;
use types::{IdGen, Type, Types};

use crate::builtin::find_builtin;

#[derive(Debug, Default, Clone)]
pub struct TypedHirGen {
    types: Types,
    _id_gen: RefCell<IdGen>,
}

impl TypedHirGen {
    pub fn gen(&self, expr: &WithMeta<Expr>) -> TypedHir {
        let Meta { id, .. } = expr.meta.as_ref().expect("must have meta");
        let ty = self.types.get(&id).expect("must have type").clone();
        let expr = match &expr.value {
            Expr::Literal(Literal::Hole) => thir::Expr::Literal(thir::Literal::Hole),
            Expr::Literal(Literal::Int(value)) => {
                thir::Expr::Literal(thir::Literal::Int(value.clone()))
            }
            Expr::Literal(Literal::Float(value)) => {
                thir::Expr::Literal(thir::Literal::Float(value.clone()))
            }
            Expr::Literal(Literal::Rational(a, b)) => {
                thir::Expr::Literal(thir::Literal::Rational(a.clone(), b.clone()))
            }
            Expr::Literal(Literal::String(value)) => {
                thir::Expr::Literal(thir::Literal::String(value.clone()))
            }
            Expr::Let {
                ty: _,
                definition,
                expression,
            } => thir::Expr::Let {
                ty: self.get_type(definition),
                definition: Box::new(self.gen(&*definition)),
                body: Box::new(self.gen(&*expression)),
            },
            Expr::Perform { input, output: _ } => thir::Expr::Perform(Box::new(self.gen(&*input))),
            Expr::Handle {
                input,
                output,
                handler,
                expr,
            } => thir::Expr::Handle {
                input: self.get_type(&input),
                output: self.get_type(&output),
                handler: Box::new(self.gen(&*handler)),
                expr: Box::new(self.gen(&*expr)),
            },
            Expr::Apply {
                function,
                arguments,
            } => {
                // TODO: lookup imported uuid to allow overwrite the builtin functions
                if let Some((builtin, params)) = find_builtin(&self.get_type(&function)) {
                    let op = thir::Expr::BuiltinOp {
                        op: builtin,
                        arguments: arguments.iter().map(|arg| self.gen(arg)).collect(),
                    };
                    if arguments.len() < params {
                        // TODO wrap by function
                        op
                    } else {
                        op
                    }
                } else {
                    if arguments.is_empty() {
                        thir::Expr::Reference
                    } else {
                        thir::Expr::Apply {
                            function: self.get_type(&function),
                            arguments: arguments.iter().map(|arg| self.gen(arg)).collect(),
                        }
                    }
                }
            }
            Expr::Product(values) => {
                thir::Expr::Product(values.iter().map(|value| self.gen(&*value)).collect())
            }
            // one ID disappeared here, but fine
            Expr::Typed { ty: _, expr } => self.gen(expr).expr,
            Expr::Function { parameter, body } => {
                let parameter = self.get_type(parameter);
                let inner = self.gen(&*body);
                if let thir::Expr::Function {
                    mut parameters,
                    body,
                } = inner.expr
                {
                    parameters.insert(0, parameter);
                    thir::Expr::Function {
                        parameters,
                        body: body,
                    }
                } else {
                    thir::Expr::Function {
                        parameters: vec![parameter],
                        body: Box::new(inner),
                    }
                }
            }
            Expr::Array(values) => {
                thir::Expr::Array(values.iter().map(|value| self.gen(&*value)).collect())
            }
            Expr::Set(values) => {
                thir::Expr::Set(values.iter().map(|value| self.gen(&*value)).collect())
            }
            Expr::Match { of, cases } => thir::Expr::Match {
                input: Box::new(self.gen(&*of)),
                cases: cases
                    .iter()
                    .map(|MatchCase { ty, expr }| thir::MatchCase {
                        ty: self.get_type(ty),
                        expr: self.gen(expr),
                    })
                    .collect(),
            },
        };
        TypedHir { id: *id, ty, expr }
    }

    fn get_type<T>(&self, expr: &WithMeta<T>) -> Type {
        self.types
            .get(&expr.meta.as_ref().expect("must have meta").id)
            .expect("must have type")
            .clone()
    }
}

#[cfg(test)]
mod tests {
    use thir::BuiltinOp;

    use super::*;
    use pretty_assertions::assert_eq;

    fn parse(input: &str) -> WithMeta<Expr> {
        use chumsky::prelude::end;
        use chumsky::{Parser, Stream};
        let expr = parser::expr::parser()
            .then_ignore(end())
            .parse(Stream::from_iter(
                input.len()..input.len() + 1,
                lexer::lexer()
                    .then_ignore(end())
                    .parse(input)
                    .unwrap()
                    .into_iter(),
            ))
            .unwrap();
        let gen = hirgen::HirGen::default();
        gen.gen(expr).unwrap()
    }

    fn infer(expr: &WithMeta<Expr>) -> Types {
        let infer = typeinfer::Ctx::default();
        infer.synth(expr).unwrap();
        infer.get_types()
    }

    #[test]
    fn literal() {
        let expr = parse("1");
        let gen = TypedHirGen {
            types: infer(&expr),
            ..Default::default()
        };
        assert_eq!(
            gen.gen(&expr),
            TypedHir {
                id: 0,
                ty: Type::Number,
                expr: thir::Expr::Literal(thir::Literal::Int(1)),
            }
        );
    }

    #[test]
    fn function_and_reference() {
        let expr = parse(r#"\ <'number>, <'string> -> <'number>"#);
        let gen = TypedHirGen {
            types: infer(&expr),
            ..Default::default()
        };
        assert_eq!(
            gen.gen(&expr),
            TypedHir {
                id: 5,
                ty: Type::Function {
                    parameters: vec![Type::Number, Type::String],
                    body: Box::new(Type::Number),
                },
                expr: thir::Expr::Function {
                    parameters: vec![Type::Number, Type::String],
                    body: Box::new(TypedHir {
                        id: 3,
                        ty: Type::Number,
                        expr: thir::Expr::Reference,
                    }),
                },
            }
        );
    }

    #[test]
    fn builtin() {
        let expr = parse(r#"<\'number, 'number -> @sum 'number> 1, 2"#);
        let gen = TypedHirGen {
            types: infer(&expr),
            ..Default::default()
        };
        assert_eq!(
            gen.gen(&expr),
            TypedHir {
                id: 8,
                ty: Type::Label {
                    label: "sum".to_string(),
                    item: Box::new(Type::Number),
                },
                expr: thir::Expr::BuiltinOp {
                    op: BuiltinOp::Add,
                    arguments: vec![
                        TypedHir {
                            id: 6,
                            ty: Type::Number,
                            expr: thir::Expr::Literal(thir::Literal::Int(1)),
                        },
                        TypedHir {
                            id: 7,
                            ty: Type::Number,
                            expr: thir::Expr::Literal(thir::Literal::Int(2)),
                        }
                    ]
                },
            }
        );
    }

    #[test]
    fn builtin_curried() {
        let expr = parse(r#"<\'number, 'number -> @sum 'number>"#);
        let _gen = TypedHirGen {
            types: infer(&expr),
            _id_gen: RefCell::new(IdGen { next: 100 }),
        };
        // TODO
        // assert_eq!(
        //     gen.gen(&expr),
        //     TypedHir {
        //         id: 8,
        //         ty: Type::Label {
        //             label: "sum".to_string(),
        //             item: Box::new(Type::Number),
        //         },
        //         expr: thir::Expr::Function {
        //             parameters: vec![
        //                 Type::Label {
        //                     label: "$$deskc 1".to_string(),
        //                     item: Box::new(Type::Number)
        //                 },
        //                 Type::Label {
        //                     label: "$$deskc 2".to_string(),
        //                     item: Box::new(Type::Number)
        //                 },
        //             ],
        //             body: Box::new(TypedHir {
        //                 id: 100,
        //                 ty: Type::Label {
        //                     label: "sum".to_string(),
        //                     item: Box::new(Type::Number),
        //                 },
        //                 expr: thir::Expr::BuiltinOp {
        //                     op: BuiltinOp::Add,
        //                     arguments: vec![
        //                         TypedHir {
        //                             id: 6,
        //                             ty: Type::Label {
        //                                 label: "$$deskc 1".to_string(),
        //                                 item: Box::new(Type::Number)
        //                             },
        //                             expr: thir::Expr::Reference,
        //                         },
        //                         TypedHir {
        //                             id: 7,
        //                             ty: Type::Label {
        //                                 label: "$$deskc 2".to_string(),
        //                                 item: Box::new(Type::Number)
        //                             },
        //                             expr: thir::Expr::Reference,
        //                         }
        //                     ]
        //                 }
        //             })
        //         },
        //     }
        // );
    }

    #[test]
    fn match_() {
        let expr = parse(
            r#"
        + 3 ~
            <'number> -> 1,
            <'string> -> "2".
        "#,
        );
        let gen = TypedHirGen {
            types: infer(&expr),
            ..Default::default()
        };
        assert_eq!(
            gen.gen(&expr),
            TypedHir {
                id: 5,
                ty: Type::Sum(vec![Type::Number, Type::String]),
                expr: thir::Expr::Match {
                    input: Box::new(TypedHir {
                        id: 0,
                        ty: Type::Number,
                        expr: thir::Expr::Literal(thir::Literal::Int(3)),
                    }),
                    cases: vec![
                        thir::MatchCase {
                            ty: Type::Number,
                            expr: TypedHir {
                                id: 2,
                                ty: Type::Number,
                                expr: thir::Expr::Literal(thir::Literal::Int(1)),
                            }
                        },
                        thir::MatchCase {
                            ty: Type::String,
                            expr: TypedHir {
                                id: 4,
                                ty: Type::String,
                                expr: thir::Expr::Literal(thir::Literal::String("2".into())),
                            }
                        },
                    ]
                },
            }
        );
    }

    // TODO: match exhaustive check
}
