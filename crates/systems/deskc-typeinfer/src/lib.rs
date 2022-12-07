pub mod ctx;
mod mono_type;
mod occurs_in;
mod polymorphic_function;
mod substitute;
mod substitute_from_ctx;
mod ty;
mod utils;
mod well_formed;

use std::{cell::RefCell, rc::Rc};

use ctx::{with_effects::WithEffects, with_type::WithType, Ctx};
use errors::typeinfer::{ExprTypeError, TypeError};
use hir::{expr::Expr, meta::WithMeta};
use ty::Type;
use types::IdGen;

pub fn synth(next_id: usize, expr: &WithMeta<Expr>) -> Result<(Ctx, Type), ExprTypeError> {
    Ctx {
        id_gen: Rc::new(RefCell::new(IdGen { next_id })),
        ..Default::default()
    }
    .synth(expr)
    .map(|WithEffects(WithType(ctx, ty), effects)| {
        assert!(ctx.continue_input.borrow().is_empty());
        assert!(ctx.continue_output.borrow().is_empty());
        let ty = ctx.substitute_from_ctx(&ty);
        let with_effects = ctx.with_effects(ty, effects);
        (ctx, with_effects)
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
    use errors::textual_diagnostics::TextualDiagnostics;
    use hir::{expr::Literal, helper::HirVisitor, meta::dummy_meta, ty::Function};
    use ids::{CardId, NodeId};
    use pretty_assertions::assert_eq;

    use crate::{
        ctx::Ctx,
        ty::{effect_expr::EffectExpr, Effect},
    };

    use super::*;

    #[allow(unused)]
    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn synth(expr: WithMeta<Expr>) -> Result<Type, ExprTypeError> {
        crate::synth(100, &expr).map(|(_, ty)| ty)
    }

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

    fn get_types(hir: &WithMeta<Expr>, ctx: &Ctx) -> Vec<(usize, Type)> {
        #[derive(Default)]
        struct HirIds {
            ids: Vec<(usize, NodeId)>,
        }
        impl HirVisitor for HirIds {
            fn visit_expr(&mut self, expr: &WithMeta<Expr>) {
                if let Some(Dson::Literal(dson::Literal::Integer(int))) = expr.meta.attrs.first() {
                    self.ids.push((*int as usize, expr.id.clone()));
                }
                self.super_visit_expr(expr);
            }
        }
        let mut hir_ids = HirIds::default();
        hir_ids.visit_expr(hir);

        let mut vec: Vec<_> = hir_ids
            .ids
            .into_iter()
            .map(|(attr, id)| (attr, ctx.get_type(&id)))
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
        assert_eq!(
            synth(dummy_meta(Expr::Literal(Literal::Integer(1)))),
            Ok(Type::Number)
        );
    }

    #[test]
    fn function() {
        assert_eq!(
            synth(dummy_meta(Expr::Apply {
                function: dummy_meta(hir::ty::Type::Function(Box::new(Function {
                    parameter: dummy_meta(hir::ty::Type::Number),
                    body: dummy_meta(hir::ty::Type::String),
                }))),
                link_name: Default::default(),
                arguments: vec![dummy_meta(Expr::Literal(Literal::Integer(1))),]
            })),
            Ok(Type::String)
        );
    }

    #[test]
    fn let_() {
        assert_eq!(
            synth(parse(
                r#"
                    $ 1; &'number
            "#
            )),
            Ok(Type::Number)
        );
    }

    #[test]
    fn generic_function() {
        let x = 102;
        assert_eq!(
            synth(parse(
                r#"
                \ x -> &x
            "#
            )),
            Ok(Type::Function {
                parameter: Box::new(Type::Existential(x)),
                body: Box::new(Type::Existential(x)),
            })
        );
    }

    #[test]
    fn let_function() {
        assert_eq!(
            synth(parse(
                r#"
                    $ \ x -> &x;
                    ^'forall a \ a -> a (1)
            "#
            )),
            Ok(Type::Number)
        );
    }

    #[test]
    fn typing_expressions() {
        let input = &r#"
            #1 $ #2 \ x -> #3 &x;
            'do #4 ^'forall a \ a -> a (#5 1);
            #6 ^'forall a \ a -> a (#7 "a")
        "#;
        let expr = &parse(input);
        let (ctx, _ty) = crate::synth(100, expr).unwrap_or_else(|error| print_error(input, error));

        assert_eq!(
            get_types(&expr, &ctx),
            vec![
                (1, Type::String),
                (
                    2,
                    Type::ForAll {
                        variable: 107,
                        bound: None,
                        body: Box::new(Type::Function {
                            parameter: Box::new(Type::Existential(107)),
                            body: Box::new(Type::Existential(107)),
                        })
                    },
                ),
                (3, Type::Existential(103)),
                (4, Type::Number),
                (5, Type::Number),
                (6, Type::String),
                (7, Type::String),
            ],
        );
    }

    #[test]
    fn subtyping_sum_in_product() {
        let expr = parse(
            r#"
            $ #1 \ +<'number, *<>> -> 1;
            #3 ^\ +<'number, *<>> -> 'number (#2 *<1, "a">)
        "#,
        );
        let (ctx, _ty) = crate::synth(100, &expr).unwrap();

        assert_eq!(
            get_types(&expr, &ctx),
            vec![
                (
                    1,
                    Type::Function {
                        parameter: Box::new(Type::Sum(vec![Type::Number, Type::Product(vec![])])),
                        body: Box::new(Type::Number),
                    },
                ),
                (2, Type::Product(vec![Type::Number, Type::String])),
                (3, Type::Number),
            ],
        );
    }

    #[test]
    fn perform() {
        let expr = parse(
            r#"
            $ #3 \ x -> #2 ^ \ 'number -> 'string (#1 ! &x ~> 'number);
            #4 ^'forall a \ a -> ! { a ~> 'number } 'string ("a")
        "#,
        );
        let (ctx, _ty) = crate::synth(100, &expr).unwrap();

        let x = ctx.get_id_of("x".into()) + 1;
        assert_eq!(
            get_types(&expr, &ctx),
            vec![
                (
                    1,
                    Type::Effectful {
                        ty: Box::new(Type::Number),
                        effects: EffectExpr::Effects(vec![Effect {
                            input: Type::Existential(x),
                            output: Type::Number,
                        }]),
                    },
                ),
                (
                    2,
                    Type::Effectful {
                        ty: Box::new(Type::String),
                        effects: EffectExpr::Effects(vec![Effect {
                            input: Type::Existential(x),
                            output: Type::Number,
                        }]),
                    },
                ),
                (
                    3,
                    Type::ForAll {
                        variable: 112,
                        bound: None,
                        body: Box::new(Type::Function {
                            parameter: Box::new(Type::Existential(112)),
                            body: Box::new(Type::Effectful {
                                ty: Box::new(Type::String),
                                effects: EffectExpr::Effects(vec![Effect {
                                    input: Type::Existential(112),
                                    output: Type::Number,
                                }]),
                            }),
                        }),
                    },
                ),
                (
                    4,
                    Type::Effectful {
                        ty: Box::new(Type::String),
                        effects: EffectExpr::Effects(vec![Effect {
                            input: Type::String,
                            output: Type::Number,
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
                  ')
                '}
                "#,
        );
        let (ctx, _ty) = crate::synth(100, &expr).unwrap();

        let x = 102;
        let y = 107;
        let z = 112;
        assert_eq!(
            get_types(&expr, &ctx),
            vec![
                (
                    1,
                    Type::Effectful {
                        ty: Box::new(Type::Existential(z)),
                        effects: EffectExpr::Effects(vec![Effect {
                            input: Type::Existential(y),
                            output: Type::Existential(z),
                        }]),
                    },
                ),
                (
                    2,
                    Type::Effectful {
                        ty: Box::new(Type::Existential(z)),
                        effects: EffectExpr::Effects(vec![Effect {
                            input: Type::Existential(x),
                            output: Type::Existential(y),
                        }]),
                    },
                ),
                (
                    3,
                    Type::Effectful {
                        ty: Box::new(Type::Existential(z)),
                        effects: EffectExpr::Effects(vec![Effect {
                            input: Type::Number,
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
            \ x -> \ y -> #3 'handle #2 ^ \'number -> y (! &x ~> 'number) '{
              x ~> 'number => #1 !<~ 1 ~> 'string
            '}
            "#,
        );
        let (ctx, _ty) = crate::synth(100, &expr).unwrap();

        assert_eq!(
            get_types(&expr, &ctx),
            vec![
                (
                    1,
                    Type::Effectful {
                        ty: Box::new(Type::String),
                        effects: EffectExpr::Effects(vec![Effect {
                            input: Type::Number,
                            output: Type::String,
                        }]),
                    },
                ),
                (
                    2,
                    Type::Effectful {
                        ty: Box::new(Type::Existential(107)),
                        effects: EffectExpr::Effects(vec![Effect {
                            input: Type::Existential(102),
                            output: Type::Number,
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
              'number ~> 'string -> ^ y 2
            '};
            #3 ^fun(
              \ @"x" 'number -> '{
                'do ! 1 ~> 'string;
                ! @"a" *<> ~> 'number,
              '}
              \ @"y" 'number -> '{
                'do ! "a" ~> 'number;
                ! @"b" *<> ~> 'number
              '}
            )
            "#;
        let expr = parse(input);
        let (ctx, _ty) = crate::synth(100, &expr).unwrap_or_else(|error| print_error(input, error));

        assert_eq!(
            get_types(&expr, &ctx),
            vec![
                (
                    1,
                    Type::Function {
                        parameter: Box::new(Type::Existential(2)),
                        body: Box::new(Type::Function {
                            parameter: Box::new(Type::Function {
                                parameter: Box::new(Type::Existential(23)),
                                body: Box::new(Type::Number)
                            }),
                            body: Box::new(Type::Effectful {
                                ty: Box::new(Type::Number),
                                effects: EffectExpr::Add(vec![])
                            })
                        })
                    }
                ),
                (
                    2,
                    Type::Effectful {
                        ty: Box::new(Type::Number),
                        effects: EffectExpr::Effects(vec![
                            Effect {
                                input: Type::Label {
                                    label: Dson::Literal(dson::Literal::String("a".into())),
                                    item: Box::new(Type::Product(vec![]))
                                },
                                output: Type::Number,
                            },
                            Effect {
                                input: Type::Label {
                                    label: Dson::Literal(dson::Literal::String("b".into())),
                                    item: Box::new(Type::Product(vec![]))
                                },
                                output: Type::Number,
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
            <@"labeled" 'number> <'number> <@"labeled" 'number> 1
        "#,
        );
        assert_eq!(
            synth(expr),
            Ok(Type::Label {
                label: "labeled".into(),
                item: Box::new(Type::Number),
            })
        );
    }

    #[test]
    fn instantiate_label() {
        let expr = parse(
            r#"
            \ x -> <@"labeled" 'number> &x
        "#,
        );
        assert_eq!(
            synth(expr),
            Ok(Type::Function {
                parameter: Box::new(Type::Label {
                    label: Dson::Literal(dson::Literal::String("labeled".into())),
                    item: Box::new(Type::Number),
                }),
                body: Box::new(Type::Label {
                    label: Dson::Literal(dson::Literal::String("labeled".into())),
                    item: Box::new(Type::Number),
                })
            })
        );
    }

    #[test]
    fn brand_supertype() {
        let expr = parse(
            r#"
            'brand "brand";
            <@"brand" 'number> 1
        "#,
        );
        assert_eq!(
            synth(expr).map_err(|e| e.error),
            Err(TypeError::NotSubtype {
                sub: types::Type::Number,
                ty: types::Type::Brand {
                    brand: Dson::Literal(dson::Literal::String("brand".into())),
                    item: Box::new(types::Type::Number),
                },
            })
        );
    }

    #[test]
    fn brand_subtype() {
        let expr = parse(
            r#"
            'brand "brand";
            <'number> &@"brand" 'number
        "#,
        );
        assert_eq!(synth(expr), Ok(Type::Number));
    }

    #[test]
    fn infer() {
        let expr = parse(
            r#"
            <_> ^ \ _ -> 'number ("a")
            "#,
        );
        let (_ctx, ty) = crate::synth(100, &expr).unwrap();

        assert_eq!(ty, Type::Number);
    }

    #[test]
    fn test_match() {
        let expr = parse(
            r#"
            \ x ->
              #2 'match #1 &x '{
                'number => <@"a" 'number> 1,
                'string => <@"b" 'number> 2,
              '}
            "#,
        );
        let (ctx, _ty) = crate::synth(100, &expr).unwrap();

        assert_eq!(
            get_types(&expr, &ctx),
            vec![
                (1, Type::Sum(vec![Type::Number, Type::String])),
                (
                    2,
                    Type::Sum(vec![
                        Type::Label {
                            label: "a".into(),
                            item: Box::new(Type::Number)
                        },
                        Type::Label {
                            label: "b".into(),
                            item: Box::new(Type::Number)
                        }
                    ])
                )
            ]
        );
    }

    // TODO:
    // Priority labels in function application
    // Priority labels in product and sum
}
