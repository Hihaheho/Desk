use std::cell::RefCell;

use hir::{
    expr::{Expr, Literal, MatchCase},
    meta::WithMeta,
};
use thir::{Handler, MapElem, TypedHir};
use types::{Effect, IdGen, Type, Types};

pub fn gen_typed_hir(next_id: usize, types: Types, expr: &WithMeta<Expr>) -> TypedHir {
    TypedHirGen {
        types,
        id_gen: RefCell::new(IdGen { next_id }),
    }
    .gen(expr)
}

#[derive(Debug, Default, Clone)]
pub struct TypedHirGen {
    types: Types,
    id_gen: RefCell<IdGen>,
}

impl TypedHirGen {
    pub fn gen(&self, expr: &WithMeta<Expr>) -> TypedHir {
        let expr_id = &expr.id;
        let ty = self.types.get(expr_id).expect("must have type").clone();
        let expr = match &expr.value {
            Expr::Literal(Literal::Hole) => todo!(),
            Expr::Literal(Literal::Integer(value)) => {
                thir::Expr::Literal(thir::Literal::Int(*value))
            }
            Expr::Literal(Literal::Float(value)) => {
                thir::Expr::Literal(thir::Literal::Float(*value))
            }
            Expr::Literal(Literal::Rational(a, b)) => {
                thir::Expr::Literal(thir::Literal::Rational(*a, *b))
            }
            Expr::Literal(Literal::String(value)) => {
                thir::Expr::Literal(thir::Literal::String(value.clone()))
            }
            Expr::Do { stmt, expr } => thir::Expr::Do {
                stmt: Box::new(self.gen(stmt)),
                expr: Box::new(self.gen(expr)),
            },
            Expr::Let {
                definition,
                expression,
            } => thir::Expr::Let {
                definition: Box::new(self.gen(definition)),
                body: Box::new(self.gen(expression)),
            },
            Expr::Perform { input, output: _ } => thir::Expr::Perform(Box::new(self.gen(input))),
            Expr::Continue { input, output: _ } => thir::Expr::Perform(Box::new(self.gen(input))),
            Expr::Handle { handlers, expr } => thir::Expr::Handle {
                handlers: handlers
                    .iter()
                    .map(
                        |hir::expr::Handler {
                             input,
                             output,
                             handler,
                         }| Handler {
                            effect: Effect {
                                input: self.get_type(input),
                                output: self.get_type(output),
                            },
                            handler: self.gen(handler),
                        },
                    )
                    .collect(),
                expr: Box::new(self.gen(expr)),
            },
            Expr::Apply {
                function,
                link_name,
                arguments,
            } => {
                // TODO: lookup imported uuid to allow overwrite the builtin functions
                thir::Expr::Apply {
                    function: self.get_type(function),
                    link_name: link_name.clone(),
                    arguments: arguments.iter().map(|arg| self.gen(arg)).collect(),
                }
            }
            Expr::Product(values) => {
                thir::Expr::Product(values.iter().map(|value| self.gen(value)).collect())
            }
            // one ID disappeared here, but fine
            Expr::Typed { ty: _, item: expr } => self.gen(expr).expr,
            Expr::Function { parameter: _, body } => {
                // get type from whole function is more accurate than from parameter.
                let function_ty = self.get_type(expr);
                if let Type::Function { parameter, body: _ } = function_ty {
                    // Flatten the function
                    thir::Expr::Function {
                        parameter: *parameter,
                        body: Box::new(self.gen(body)),
                    }
                } else {
                    panic!("function is inferred to not function??");
                }
            }
            Expr::Vector(values) => {
                thir::Expr::Vector(values.iter().map(|value| self.gen(value)).collect())
            }
            Expr::Map(elems) => thir::Expr::Map(
                elems
                    .iter()
                    .map(|elem| MapElem {
                        key: self.gen(&elem.key),
                        value: self.gen(&elem.value),
                    })
                    .collect(),
            ),
            Expr::Match { of, cases } => thir::Expr::Match {
                input: Box::new(self.gen(of)),
                cases: cases
                    .iter()
                    .map(|MatchCase { ty, expr }| thir::MatchCase {
                        ty: self.get_type(ty),
                        expr: self.gen(expr),
                    })
                    .collect(),
            },
            Expr::Label {
                label, item: body, ..
            }
            | Expr::Brand {
                brand: label,
                item: body,
                ..
            } => thir::Expr::Label {
                label: label.clone(),
                item: Box::new(self.gen(body)),
            },
        };
        TypedHir {
            id: expr_id.clone(),
            ty,
            expr,
        }
    }

    fn get_type<T>(&self, expr: &WithMeta<T>) -> Type {
        self.types.get(&expr.id).expect("must have type").clone()
    }

    pub fn next_id(&self) -> usize {
        self.id_gen.borrow_mut().next_id()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use ids::{CardId, NodeId};
    use thir::visitor::TypedHirVisitorMut;

    use super::*;
    use pretty_assertions::assert_eq;

    fn parse(input: &str) -> WithMeta<Expr> {
        use deskc::card::CardQueries;
        use deskc::{Code, SyntaxKind};
        let card_id = CardId::new();
        let mut compiler = deskc::card::CardsCompiler::default();
        compiler.set_code(
            card_id.clone(),
            Code::SourceCode {
                syntax: SyntaxKind::Minimalist,
                source: Arc::new(input.to_string()),
            },
        );
        compiler.hir(card_id).unwrap().hir.clone()
    }

    fn infer(expr: &WithMeta<Expr>) -> Types {
        let infer = typeinfer::ctx::Ctx::default();
        let _ = infer.synth(expr).unwrap();
        infer.get_types()
    }

    pub struct RemoveIdVisitor;
    impl TypedHirVisitorMut for RemoveIdVisitor {
        fn visit(&mut self, hir: &mut TypedHir) {
            hir.id = NodeId::default();
            self.super_visit(hir);
        }
    }
    fn remove_id(mut expr: TypedHir) -> TypedHir {
        RemoveIdVisitor.visit(&mut expr);
        expr
    }

    #[test]
    fn literal() {
        let expr = parse("1");
        let gen = TypedHirGen {
            types: infer(&expr),
            ..Default::default()
        };
        assert_eq!(
            remove_id(gen.gen(&expr)),
            TypedHir {
                id: NodeId::default(),
                ty: Type::Number,
                expr: thir::Expr::Literal(thir::Literal::Int(1)),
            }
        );
    }

    #[test]
    fn function_and_reference() {
        let expr = parse(r#"\ 'number -> \ 'string -> &'number"#);
        let gen = TypedHirGen {
            types: infer(&expr),
            ..Default::default()
        };
        assert_eq!(
            remove_id(gen.gen(&expr)),
            TypedHir {
                id: NodeId::default(),
                ty: Type::Function {
                    parameter: Box::new(Type::Number),
                    body: Box::new(Type::Function {
                        parameter: Box::new(Type::String),
                        body: Box::new(Type::Number)
                    }),
                },
                expr: thir::Expr::Function {
                    parameter: Type::Number,
                    body: Box::new(TypedHir {
                        id: NodeId::default(),
                        ty: Type::Number,
                        expr: thir::Expr::Apply {
                            function: Type::Number,
                            link_name: Default::default(),
                            arguments: vec![]
                        },
                    }),
                },
            }
        );
    }

    #[test]
    fn match_() {
        let expr = parse(
            r#"
        + 3 ~
          'number -> 1,
          'string -> "2".
        "#,
        );
        let gen = TypedHirGen {
            types: infer(&expr),
            ..Default::default()
        };
        assert_eq!(
            remove_id(gen.gen(&expr)),
            TypedHir {
                id: NodeId::default(),
                ty: Type::Sum(vec![Type::Number, Type::String]),
                expr: thir::Expr::Match {
                    input: Box::new(TypedHir {
                        id: NodeId::default(),
                        ty: Type::Number,
                        expr: thir::Expr::Literal(thir::Literal::Int(3)),
                    }),
                    cases: vec![
                        thir::MatchCase {
                            ty: Type::Number,
                            expr: TypedHir {
                                id: NodeId::default(),
                                ty: Type::Number,
                                expr: thir::Expr::Literal(thir::Literal::Int(1)),
                            }
                        },
                        thir::MatchCase {
                            ty: Type::String,
                            expr: TypedHir {
                                id: NodeId::default(),
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
