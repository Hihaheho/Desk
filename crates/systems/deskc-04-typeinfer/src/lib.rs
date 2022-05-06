pub mod ctx;
pub mod error;
mod mono_type;
mod occurs_in;
mod polymorphic_function;
mod substitute;
mod substitute_from_ctx;
mod ty;
mod utils;
mod well_formed;
mod with_effects;

use std::{cell::RefCell, rc::Rc};

use ctx::Ctx;
use error::{ExprTypeError, TypeError};
use hir::{expr::Expr, meta::WithMeta};
use ty::Type;
use types::IdGen;
use with_effects::WithEffects;

pub fn synth(next_id: usize, expr: &WithMeta<Expr>) -> Result<(Ctx, Type), ExprTypeError> {
    Ctx {
        id_gen: Rc::new(RefCell::new(IdGen { next_id })),
        ..Default::default()
    }
    .synth(expr)
    .map(|WithEffects((ctx, ty), effects)| {
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
    use ariadne::{Label, Report, ReportKind, Source};
    use file::FileId;
    use hir::{expr::Literal, meta::dummy_meta};
    use hirgen::HirGen;
    use pretty_assertions::assert_eq;
    use textual_diagnostics::TextualDiagnostics;

    use crate::{
        ctx::Ctx,
        ty::{effect_expr::EffectExpr, Effect},
    };

    use super::*;

    fn synth(expr: WithMeta<Expr>) -> Result<Type, ExprTypeError> {
        crate::synth(100, &expr).map(|(_, ty)| ty)
    }

    fn parse(input: &str) -> WithMeta<Expr> {
        parse_inner(input).1
    }

    fn parse_inner(input: &str) -> (HirGen, WithMeta<Expr>) {
        let tokens = lexer::scan(input).unwrap();
        let ast = parser::parse(tokens).unwrap();
        hirgen::gen_hir(FileId(0), &ast, Default::default()).unwrap()
    }

    fn get_types(hirgen: &HirGen, ctx: &Ctx) -> Vec<(usize, Type)> {
        let mut vec: Vec<_> = hirgen
            .attrs
            .borrow()
            .iter()
            .flat_map(|(id, attrs)| {
                attrs.iter().map(|attr| match attr {
                    Expr::Literal(Literal::Int(attr)) => (*attr as usize, *id),
                    _ => todo!(),
                })
            })
            .map(|(attr, id)| (attr, ctx.get_type(&id)))
            .collect();

        vec.sort_by_key(|(attr, _)| *attr);
        vec
    }

    fn print_error<T>(input: &str, error: ExprTypeError) -> T {
        let diagnostics: TextualDiagnostics = error.into();
        let report = Report::build(ReportKind::Error, (), 0).with_message(diagnostics.title);
        diagnostics
            .reports
            .into_iter()
            .fold(
                report,
                |report, textual_diagnostics::Report { span, text }| {
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
            synth(dummy_meta(Expr::Literal(Literal::Int(1)))),
            Ok(Type::Number)
        );
    }

    #[test]
    fn function() {
        assert_eq!(
            synth(dummy_meta(Expr::Apply {
                function: dummy_meta(hir::ty::Type::Function {
                    parameter: Box::new(dummy_meta(hir::ty::Type::Number)),
                    body: Box::new(dummy_meta(hir::ty::Type::String)),
                }),
                link_name: Default::default(),
                arguments: vec![dummy_meta(Expr::Literal(Literal::Int(1))),]
            })),
            Ok(Type::String)
        );
    }

    #[test]
    fn let_() {
        assert_eq!(
            synth(parse(
                r#"
                    $ 1 ~ &'number
            "#
            )),
            Ok(Type::Number)
        );
    }

    #[test]
    fn let_with_type() {
        assert_eq!(
            synth(parse(
                r#"
                    $ 1: 'a x ~ &'a x
            "#
            )),
            Ok(Type::Number)
        );
    }

    #[test]
    fn generic_function() {
        assert_eq!(
            synth(parse(
                r#"
                    \ 'a x -> &'a x
            "#
            )),
            Ok(Type::Function {
                parameter: Box::new(Type::Existential(101)),
                body: Box::new(Type::Existential(101)),
            })
        );
    }

    #[test]
    fn let_function() {
        assert_eq!(
            synth(parse(
                r#"
                    $ \ 'a x -> &'a x: 'a id ~
                    >'a id 1
            "#
            )),
            Ok(Type::Number)
        );
    }

    #[test]
    fn typing_expressions() {
        let input = &r#"
            #1 $ #2 \ 'a x -> #3 &'a x: 'a id ~
            $ #4 >'a id #5 1 ~
            #6 >'a id #7 "a"
        "#;
        let (hirgen, expr) = parse_inner(input);
        let (ctx, _ty) = crate::synth(100, &expr).unwrap_or_else(|error| print_error(input, error));

        assert_eq!(
            get_types(&hirgen, &ctx),
            vec![
                (1, Type::String),
                (
                    2,
                    Type::Function {
                        parameter: Box::new(Type::Existential(102)),
                        body: Box::new(Type::Existential(102)),
                    },
                ),
                (3, Type::Existential(102)),
                (4, Type::Number),
                (5, Type::Number),
                (6, Type::String),
                (7, Type::String),
            ],
        );
    }

    #[test]
    fn subtyping_sum_in_product() {
        let (hirgen, expr) = parse_inner(
            r#"
            $ #1 \ + 'number, * -> 1: 'a fun ~
            #3 >'a fun #2 * 1, "a"
        "#,
        );
        let (ctx, _ty) = crate::synth(100, &expr).unwrap();

        assert_eq!(
            get_types(&hirgen, &ctx),
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
        let (hirgen, expr) = parse_inner(
            r#"
            $ #3 \ x -> #2 > \ 'number -> 'string ~ #1 ! &x => 'number: fun ~
            #4 >fun "a"
        "#,
        );
        let (ctx, _ty) = crate::synth(100, &expr).unwrap();

        assert_eq!(
            get_types(&hirgen, &ctx),
            vec![
                (
                    1,
                    Type::Effectful {
                        ty: Box::new(Type::Number),
                        effects: EffectExpr::Effects(vec![Effect {
                            input: Type::Existential(102),
                            output: Type::Number,
                        }]),
                    },
                ),
                (
                    2,
                    Type::Effectful {
                        ty: Box::new(Type::String),
                        effects: EffectExpr::Effects(vec![Effect {
                            input: Type::Existential(102),
                            output: Type::Number,
                        }]),
                    },
                ),
                (
                    3,
                    Type::Function {
                        parameter: Box::new(Type::Existential(102)),
                        body: Box::new(Type::Effectful {
                            ty: Box::new(Type::String),
                            effects: EffectExpr::Effects(vec![Effect {
                                input: Type::Existential(102),
                                output: Type::Number,
                            }]),
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
        let (hirgen, expr) = parse_inner(
            r#"
                    \ x, y, z ->
                      #3 'handle #2 > \y -> z ! &x => y ~
                      x => y ->
                        $ ! 1 => 'string ~
                        #1 ! &y => z
                "#,
        );
        let (ctx, _ty) = crate::synth(100, &expr).unwrap();

        // x: 1, y: 5, z: 9
        assert_eq!(
            get_types(&hirgen, &ctx),
            vec![
                (
                    1,
                    Type::Effectful {
                        ty: Box::new(Type::Existential(109)),
                        effects: EffectExpr::Effects(vec![Effect {
                            input: Type::Existential(105),
                            output: Type::Existential(109),
                        }]),
                    },
                ),
                (
                    2,
                    Type::Effectful {
                        ty: Box::new(Type::Existential(109)),
                        effects: EffectExpr::Effects(vec![Effect {
                            input: Type::Existential(101),
                            output: Type::Existential(105),
                        }]),
                    },
                ),
                (
                    3,
                    Type::Effectful {
                        ty: Box::new(Type::Existential(109)),
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
        let (hirgen, expr) = parse_inner(
            r#"
            \x, y ->
              #3 'handle #2 > \'number -> 'string ! &x => 'number ~
              x => 'number ->
                #1 <! &y
            "#,
        );
        let (ctx, _ty) = crate::synth(100, &expr).unwrap();

        assert_eq!(
            get_types(&hirgen, &ctx),
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
                        ty: Box::new(Type::String),
                        effects: EffectExpr::Effects(vec![Effect {
                            input: Type::Existential(101),
                            output: Type::Number,
                        }]),
                    },
                ),
                (3, Type::String),
            ]
        );
    }

    #[test]
    fn test_continue_with_output() {
        let (hirgen, expr) = parse_inner(
            r#"
            \x, y ->
              #3 'handle #2 > \'number -> y ! &x => 'number ~
              x => 'number ->
                #1 <! 1 => 'string
            "#,
        );
        let (ctx, _ty) = crate::synth(100, &expr).unwrap();

        assert_eq!(
            get_types(&hirgen, &ctx),
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
                        ty: Box::new(Type::Existential(105)),
                        effects: EffectExpr::Effects(vec![Effect {
                            input: Type::Existential(101),
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
            $ #1 \x, y ->
              ^#2 'handle > x 1 ~
              'number => 'string ->
                > y 2
              : 'number
            : fun ~
            #3 >fun
              \ @x 'number ->
                $ ! 1 => 'string ~
                ! @a * => 'number,
              \ @y 'number ->
                $ ! "a" => 'number ~
                ! @b * => 'number
            "#;
        let (hirgen, expr) = parse_inner(input);
        let (ctx, _ty) = crate::synth(100, &expr).unwrap_or_else(|error| print_error(input, error));

        assert_eq!(
            get_types(&hirgen, &ctx),
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
                                    label: "a".into(),
                                    item: Box::new(Type::Product(vec![]))
                                },
                                output: Type::Number,
                            },
                            Effect {
                                input: Type::Label {
                                    label: "b".into(),
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
            ^^^1: @labeled 'number: 'number: @labeled 'number
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
            \ 'a x -> ^&'a x: @labeled 'number
        "#,
        );
        assert_eq!(
            synth(expr),
            Ok(Type::Function {
                parameter: Box::new(Type::Label {
                    label: "labeled".into(),
                    item: Box::new(Type::Number),
                }),
                body: Box::new(Type::Label {
                    label: "labeled".into(),
                    item: Box::new(Type::Number),
                })
            })
        );
    }

    #[test]
    fn brand_supertype() {
        let expr = parse(
            r#"
            'brand brand
            ^1: @brand 'number
        "#,
        );
        assert_eq!(
            synth(expr).map_err(|e| e.error),
            Err(TypeError::NotSubtype {
                sub: Type::Number,
                ty: Type::Brand {
                    brand: "brand".into(),
                    item: Box::new(Type::Number),
                },
            })
        );
    }

    #[test]
    fn brand_subtype() {
        let expr = parse(
            r#"
            'brand brand
            ^&@brand 'number: 'number
        "#,
        );
        assert_eq!(synth(expr), Ok(Type::Number));
    }

    #[test]
    fn infer() {
        let (_hirgen, expr) = parse_inner(
            r#"
            ^> \ _ -> 'number "a": _
            "#,
        );
        let (_ctx, ty) = crate::synth(100, &expr).unwrap();

        assert_eq!(ty, Type::Number);
    }

    #[test]
    fn test_match() {
        let (hirgen, expr) = parse_inner(
            r#"
            \ 'a x ->
              #2 + #1 &'a x ~
               'number -> ^1: @a 'number,
               'string -> ^2: @b 'number.
            "#,
        );
        let (ctx, _ty) = crate::synth(100, &expr).unwrap();

        assert_eq!(
            get_types(&hirgen, &ctx),
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
