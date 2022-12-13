mod cast_strategies;
pub mod ctx;
mod internal_type;
mod mappings;
mod mono_type;
mod occurs_in;
mod partial_ord_max;
mod polymorphic_function;
mod similarity;
mod substitute;
mod substitute_from_ctx;
mod utils;
mod well_formed;

use std::{cell::RefCell, rc::Rc};

use cast_strategies::CastStrategy;
use ctx::Ctx;
use errors::typeinfer::{ExprTypeError, TypeError};
use hir::{expr::Expr, meta::WithMeta};
use ty::conclusion::{TypeConclusions, TypeToType};
use utils::IdGen;

pub fn synth(next_id: usize, expr: &WithMeta<Expr>) -> Result<TypeConclusions, ExprTypeError> {
    let ctx = Ctx {
        id_gen: Rc::new(RefCell::new(IdGen { next_id })),
        ..Default::default()
    }
    .synth(expr)?
    .0
     .0;
    let types = expr
        .get_expr_ids()
        .filter_map(|id| {
            let ty = ctx.gen_type(&ctx.get_type(&id).ok()?).ok()?;
            Some((id, ty))
        })
        .collect();
    let cast_strategies = ctx
        .cast_strategies
        .borrow()
        .iter()
        .filter_map(|(type_to_type, strategy)| {
            let type_to_type = TypeToType {
                from: ctx.gen_type(&type_to_type.0).ok()?,
                to: ctx.gen_type(&type_to_type.1).ok()?,
            };
            let cast_strategy = match strategy {
                CastStrategy::ProductToProduct(mapping) => {
                    ty::conclusion::CastStrategy::ProductToProduct(
                        mapping
                            .iter()
                            .filter_map(|(from, to)| {
                                let from = ctx.gen_type(from).ok()?;
                                let to = ctx.gen_type(to).ok()?;
                                Some((from, to))
                            })
                            .collect(),
                    )
                }
                CastStrategy::SumToSum(mapping) => ty::conclusion::CastStrategy::SumToSum(
                    mapping
                        .iter()
                        .filter_map(|(from, to)| {
                            let from = ctx.gen_type(from).ok()?;
                            let to = ctx.gen_type(to).ok()?;
                            Some((from, to))
                        })
                        .collect(),
                ),
                CastStrategy::ProductToInner(ty) => {
                    ty::conclusion::CastStrategy::ProductToInner(ctx.gen_type(ty).ok()?)
                }
                CastStrategy::InnerToSum(ty) => {
                    ty::conclusion::CastStrategy::InnerToSum(ctx.gen_type(ty).ok()?)
                }
            };
            Some((type_to_type, cast_strategy))
        })
        .collect();
    Ok(TypeConclusions {
        types,
        cast_strategies,
    })
}

fn to_expr_type_error(expr: &WithMeta<Expr>, error: TypeError) -> ExprTypeError {
    ExprTypeError {
        meta: expr.meta.clone(),
        error,
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use ariadne::{Label, Report, ReportKind, Source};
    use dson::Dson;
    use errors::{textual_diagnostics::TextualDiagnostics, typeinfer::TypeOrString};
    use hir::visitor::HirVisitor;
    use ids::{Entrypoint, FileId, NodeId};
    use pretty_assertions::assert_eq;
    use ty::{
        conclusion::{CastStrategy, TypeToType},
        Effect, EffectExpr, Function, Type,
    };

    use super::*;

    #[allow(unused)]
    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn synth(expr: &WithMeta<Expr>) -> Result<TypeConclusions, ExprTypeError> {
        crate::synth(100, &expr)
    }

    fn parse(input: &str) -> WithMeta<Expr> {
        use deskc::card::DeskcQueries;
        use deskc::{Code, SyntaxKind};
        let file_id = FileId::new();
        let mut compiler = deskc::card::DeskCompiler::default();
        compiler.set_code(
            file_id.clone(),
            Code::SourceCode {
                syntax: SyntaxKind::Minimalist,
                source: Arc::new(input.to_string()),
            },
        );
        compiler
            .hir(Entrypoint::File(file_id))
            .unwrap()
            .as_ref()
            .clone()
    }

    fn get_types(hir: &WithMeta<Expr>, ctx: &TypeConclusions) -> Vec<(usize, Type)> {
        #[derive(Default)]
        struct HirIds {
            ids: Vec<(usize, NodeId)>,
        }
        impl HirVisitor for HirIds {
            fn visit_expr(&mut self, expr: &WithMeta<Expr>) {
                if let Some(Dson::Literal(dson::Literal::Integer(int))) = expr.meta.attrs.first() {
                    self.ids.push((*int as usize, expr.meta.id.clone()));
                }
                self.super_visit_expr(expr);
            }
        }
        let mut hir_ids = HirIds::default();
        hir_ids.visit_expr(hir);

        let mut vec: Vec<_> = hir_ids
            .ids
            .into_iter()
            .map(|(attr, id)| (attr, ctx.get_type(&id).unwrap().clone()))
            .collect();

        vec.sort_by_key(|(attr, _)| *attr);
        vec
    }

    fn print_error<T>(input: &str, error: ExprTypeError) -> T {
        let diagnostics: TextualDiagnostics = (&error).into();
        let report = Report::build(ReportKind::Error, (), 0).with_message(diagnostics.title);
        diagnostics
            .reports
            .into_iter()
            .fold(
                report,
                |report, errors::textual_diagnostics::Report { span, text }| {
                    report.with_label(Label::new(span).with_message(text))
                },
            )
            .finish()
            .print(Source::from(input))
            .unwrap();
        panic!()
    }

    #[test]
    fn number() {
        let expr = parse(
            r#"
                #1 1
            "#,
        );
        let conclusion = synth(&expr).unwrap();
        assert_eq!(get_types(&expr, &conclusion), [(1, Type::Integer)]);
    }

    #[test]
    fn function() {
        let expr = parse(
            r#"
                #1 ^\ 'integer -> 'string (1)
            "#,
        );
        let conclusion = synth(&expr).unwrap();
        assert_eq!(get_types(&expr, &conclusion), [(1, Type::String)]);
    }

    #[test]
    fn let_() {
        let expr = parse(
            r#"
                  #1  $ 1; &'integer
            "#,
        );
        let conclusion = synth(&expr).unwrap();
        assert_eq!(get_types(&expr, &conclusion), [(1, Type::Integer)]);
    }

    #[test]
    fn generic_function() {
        let expr = parse(
            r#"
                  #1 \x -> &x
            "#,
        );
        let conclusion = synth(&expr).unwrap();
        assert_eq!(
            get_types(&expr, &conclusion),
            [(
                1,
                Type::Function(Box::new(ty::Function {
                    parameter: Type::Variable("a".into()),
                    body: Type::Variable("a".into()),
                })),
            )]
        );
    }

    #[test]
    fn let_function() {
        let expr = parse(
            r#"
                #1 $ \ x -> &x;
                ^'forall a \ a -> a (1)
            "#,
        );
        let conclusion = synth(&expr).unwrap();
        assert_eq!(get_types(&expr, &conclusion), [(1, Type::Integer)]);
    }

    #[test]
    fn typing_expressions() {
        let input = &r#"
            #1 $ #2 \ x -> #3 &x;
            'do #4 ^'forall a \ a -> a (#5 1);
            #6 ^'forall a \ a -> a (#7 "a")
        "#;
        let expr = &parse(input);
        let conclusion = crate::synth(100, expr).unwrap_or_else(|error| print_error(input, error));

        assert_eq!(
            get_types(&expr, &conclusion),
            vec![
                (1, Type::String),
                (
                    2,
                    Type::ForAll {
                        variable: "a".into(),
                        bound: None,
                        body: Box::new(Type::Function(Box::new(Function {
                            parameter: Type::Variable("a".into()),
                            body: Type::Variable("a".into()),
                        })))
                    },
                ),
                (3, Type::Variable("b".into())),
                (4, Type::Integer),
                (5, Type::Integer),
                (6, Type::String),
                (7, Type::String),
            ],
        );
    }

    #[test]
    fn subtyping_sum_in_product() {
        let expr = parse(
            r#"
            $ #1 \ +<'integer, *<>> -> 1;
            #3 ^\ +<'integer, *<>> -> 'integer (#2 *<1, "a">)
        "#,
        );
        let conclusion = crate::synth(100, &expr).unwrap();

        assert_eq!(
            get_types(&expr, &conclusion),
            vec![
                (
                    1,
                    Type::Function(Box::new(Function {
                        parameter: Type::Sum(vec![Type::Integer, Type::Product(vec![])]),
                        body: Type::Integer,
                    })),
                ),
                (2, Type::Product(vec![Type::Integer, Type::String])),
                (3, Type::Integer),
            ],
        );
    }

    #[test]
    fn perform() {
        let expr = parse(
            r#"
            $ #3 \ x -> #2 ^ \ 'integer -> 'string (#1 ! &x ~> 'integer);
            #4 ^'forall a \ a -> ! { a ~> 'integer } 'string ("a")
        "#,
        );
        let conclusion = crate::synth(100, &expr).unwrap();

        assert_eq!(
            get_types(&expr, &conclusion),
            vec![
                (
                    1,
                    Type::Effectful {
                        ty: Box::new(Type::Integer),
                        effects: EffectExpr::Effects(vec![Effect {
                            input: Type::Variable("b".into()),
                            output: Type::Integer,
                        }]),
                    },
                ),
                (
                    2,
                    Type::Effectful {
                        ty: Box::new(Type::String),
                        effects: EffectExpr::Effects(vec![Effect {
                            input: Type::Variable("b".into()),
                            output: Type::Integer,
                        }]),
                    },
                ),
                (
                    3,
                    Type::ForAll {
                        variable: "a".into(),
                        bound: None,
                        body: Box::new(Type::Function(Box::new(Function {
                            parameter: Type::Variable("a".into()),
                            body: Type::Effectful {
                                ty: Box::new(Type::String),
                                effects: EffectExpr::Effects(vec![Effect {
                                    input: Type::Variable("a".into()),
                                    output: Type::Integer,
                                }]),
                            },
                        }))),
                    },
                ),
                (
                    4,
                    Type::Effectful {
                        ty: Box::new(Type::String),
                        effects: EffectExpr::Effects(vec![Effect {
                            input: Type::String,
                            output: Type::Integer,
                        }]),
                    }
                ),
            ],
        );
    }

    #[test]
    fn handle() {
        let expr = parse(
            r#"
                \ x -> \ y -> \ z -> #3 'handle #2 ^ \ y -> z (! &x ~> y) '{
                  x ~> y => '(
                    'do ! 1 ~> 'string;
                    #1 ! &y ~> z
                  )'
                }'
                "#,
        );
        let conclusion = crate::synth(100, &expr).unwrap();

        assert_eq!(
            get_types(&expr, &conclusion),
            vec![
                (
                    1,
                    Type::Effectful {
                        ty: Box::new(Type::Variable("c".into())),
                        effects: EffectExpr::Effects(vec![Effect {
                            input: Type::Variable("b".into()),
                            output: Type::Variable("c".into()),
                        }]),
                    },
                ),
                (
                    2,
                    Type::Effectful {
                        ty: Box::new(Type::Variable("c".into())),
                        effects: EffectExpr::Effects(vec![Effect {
                            input: Type::Variable("a".into()),
                            output: Type::Variable("b".into()),
                        }]),
                    },
                ),
                (
                    3,
                    Type::Effectful {
                        ty: Box::new(Type::Variable("c".into())),
                        effects: EffectExpr::Effects(vec![Effect {
                            input: Type::Integer,
                            output: Type::String,
                        }]),
                    },
                ),
            ],
        );
    }

    #[test]
    fn test_continue() {
        let expr = parse(
            r#"
            \ x -> \ y -> #3 'handle #2 ^ \'integer -> y (! &x ~> 'integer) '{
              x ~> 'integer => #1 !<~ 1 ~> 'string
            }'
            "#,
        );
        let conclusion = synth(&expr).unwrap();

        assert_eq!(
            get_types(&expr, &conclusion),
            vec![
                (
                    1,
                    Type::Effectful {
                        ty: Box::new(Type::String),
                        effects: EffectExpr::Effects(vec![Effect {
                            input: Type::Integer,
                            output: Type::String,
                        }]),
                    },
                ),
                (
                    2,
                    Type::Effectful {
                        ty: Box::new(Type::String),
                        effects: EffectExpr::Effects(vec![Effect {
                            input: Type::Variable("a".into()),
                            output: Type::Integer,
                        }]),
                    },
                ),
                (3, Type::String),
            ]
        );
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_polymorphic_effectful() {
        let input = r#"
            $ #1 \ x -> \ y -> ^#2 'handle ^ x 1 '{
              'integer ~> 'string -> ^ y 2
            }';
            #3 ^fun(
              \ @"x" 'integer -> '{
                'do ! 1 ~> 'string;
                ! @"a" *<> ~> 'integer,
              }'
              \ @"y" 'integer -> '{
                'do ! "a" ~> 'integer;
                ! @"b" *<> ~> 'integer
              }'
            )
            "#;
        let expr = parse(input);
        let conclusion = synth(&expr).unwrap();

        assert_eq!(
            get_types(&expr, &conclusion),
            vec![
                (
                    1,
                    Type::Function(Box::new(Function {
                        parameter: Type::Variable("a".into()),
                        body: Type::Function(Box::new(Function {
                            parameter: Type::Function(Box::new(Function {
                                parameter: Type::Variable("a".into()),
                                body: Type::Integer
                            })),
                            body: Type::Effectful {
                                ty: Box::new(Type::Integer),
                                effects: EffectExpr::Add(vec![])
                            }
                        }))
                    }))
                ),
                (
                    2,
                    Type::Effectful {
                        ty: Box::new(Type::Integer),
                        effects: EffectExpr::Effects(vec![
                            Effect {
                                input: Type::Label {
                                    label: Dson::Literal(dson::Literal::String("a".into())),
                                    item: Box::new(Type::Product(vec![]))
                                },
                                output: Type::Integer,
                            },
                            Effect {
                                input: Type::Label {
                                    label: Dson::Literal(dson::Literal::String("b".into())),
                                    item: Box::new(Type::Product(vec![]))
                                },
                                output: Type::Integer,
                            }
                        ]),
                    },
                ),
            ]
        );
    }

    #[test]
    fn label() {
        let expr = parse(
            r#"
            #1 <@"labeled" 'integer> <'integer> <@"labeled" 'integer> 1
        "#,
        );
        let conclusion = synth(&expr).unwrap();
        assert_eq!(
            get_types(&expr, &conclusion),
            vec![(
                1,
                Type::Label {
                    label: Dson::Literal(dson::Literal::String("labeled".into())),
                    item: Box::new(Type::Integer),
                },
            ),]
        );
    }

    #[test]
    fn instantiate_label() {
        let expr = parse(
            r#"
            #1 \ x -> <@"labeled" 'integer> &x
        "#,
        );
        let conclusion = synth(&expr).unwrap();
        assert_eq!(
            get_types(&expr, &conclusion),
            vec![(
                1,
                Type::Function(Box::new(Function {
                    parameter: Type::Label {
                        label: Dson::Literal(dson::Literal::String("labeled".into())),
                        item: Box::new(Type::Integer),
                    },
                    body: Type::Label {
                        label: Dson::Literal(dson::Literal::String("labeled".into())),
                        item: Box::new(Type::Integer),
                    },
                })),
            ),]
        );
    }

    #[test]
    fn brand_supertype() {
        let expr = parse(
            r#"
            'brand "brand";
            <@"brand" 'integer> 1
        "#,
        );
        assert_eq!(
            synth(&expr).map_err(|e| e.error),
            Err(TypeError::NotSubtype {
                sub: ty::Type::Integer.into(),
                ty: ty::Type::Brand {
                    brand: Dson::Literal(dson::Literal::String("brand".into())),
                    item: Box::new(ty::Type::Integer),
                }
                .into(),
            })
        );
    }

    #[test]
    fn brand_subtype() {
        let expr = parse(
            r#"
            'brand "brand";
            #1 <'integer> &@"brand" 'integer
        "#,
        );
        let conclusion = synth(&expr).unwrap();
        assert_eq!(get_types(&expr, &conclusion), vec![(1, Type::Integer)]);
    }

    #[test]
    fn infer() {
        let expr = parse(
            r#"
            #1 <_> ^ \ _ -> 'integer ("a")
            "#,
        );
        let conclusion = synth(&expr).unwrap();

        assert_eq!(get_types(&expr, &conclusion), vec![(1, Type::Integer),]);
    }

    #[test]
    fn test_match() {
        let expr = parse(
            r#"
            \ x ->
              #2 'match #1 &x '{
                'integer => <@"a" 'integer> 1,
                'string => <@"b" 'integer> 2,
              }'
            "#,
        );
        let conclusion = synth(&expr).unwrap();

        assert_eq!(
            get_types(&expr, &conclusion),
            vec![
                (1, Type::Sum(vec![Type::Integer, Type::String])),
                (
                    2,
                    Type::Sum(vec![
                        Type::Label {
                            label: "a".into(),
                            item: Box::new(Type::Integer)
                        },
                        Type::Label {
                            label: "b".into(),
                            item: Box::new(Type::Integer)
                        }
                    ])
                )
            ]
        );
    }

    #[test]
    fn test_numbers() {
        init();
        let expr = parse("#1 *<1, 2.0, 3 / 4>");
        let conclusion = synth(&expr).unwrap();

        assert_eq!(
            get_types(&expr, &conclusion),
            vec![(
                1,
                Type::Product(vec![Type::Integer, Type::Real, Type::Rational])
            )]
        );
    }

    #[test]
    fn test_integer_to_rational() {
        let expr = parse(
            r#"
            #1 <'rational> 1
            "#,
        );
        let conclusion = synth(&expr).unwrap();
        assert_eq!(get_types(&expr, &conclusion), vec![(1, Type::Rational)]);
    }

    #[test]
    fn test_rational_to_integer() {
        let expr = parse(
            r#"
                    #1 <'integer> 1 / 2
                    "#,
        );

        assert!(matches!(
            crate::synth(100, &expr),
            Err(ExprTypeError {
                meta: _,
                error: TypeError::NotSubtype {
                    sub: TypeOrString::Type(ty::Type::Rational),
                    ty: TypeOrString::Type(ty::Type::Integer),
                },
            })
        ));
    }

    #[test]
    fn test_cast_strategy_product_to_type() {
        let expr = parse(
            r#"
            #1 <@"l" 'real> *<@"l" 1, @"r" 2>
            "#,
        );
        let conclusion = synth(&expr).unwrap();
        assert_eq!(
            conclusion
                .cast_strategies
                .get(&TypeToType {
                    from: Type::Product(vec![
                        Type::Label {
                            label: "l".into(),
                            item: Box::new(Type::Integer)
                        },
                        Type::Label {
                            label: "r".into(),
                            item: Box::new(Type::Integer)
                        }
                    ]),
                    to: Type::Label {
                        label: "l".into(),
                        item: Box::new(Type::Real)
                    }
                })
                .expect("cast strategy not found"),
            &CastStrategy::ProductToInner(Type::Label {
                label: "l".into(),
                item: Box::new(Type::Integer)
            })
        );
    }
}
