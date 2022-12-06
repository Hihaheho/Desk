mod assertion;
mod test_case;

macro_rules! test {
    ($case:ident) => {
        #[test]
        fn $case() {
            let _ = env_logger::builder().is_test(true).try_init();
            fn print_errors<T>(input: &str, error: deskc::query_result::QueryError) -> T {
                use ariadne::{Label, Report, ReportKind, Source};
                use errors::textual_diagnostics::{Report as TDReport, TextualDiagnostics};
                let diagnostics: TextualDiagnostics = if let Some(syntax_error) =
                    error.downcast_ref::<errors::syntax::SyntaxError>()
                {
                    syntax_error.into()
                } else if let Some(typeinfer_error) =
                    error.downcast_ref::<errors::typeinfer::ExprTypeError>()
                {
                    typeinfer_error.into()
                } else {
                    panic!("unexpected error: {:?}", error);
                };
                let report =
                    Report::build(ReportKind::Error, (), 0).with_message(diagnostics.title);
                diagnostics
                    .reports
                    .into_iter()
                    .fold(report, |report, TDReport { span, text }| {
                        report.with_label(Label::new(span).with_message(text))
                    })
                    .finish()
                    .print(Source::from(input))
                    .unwrap();
                panic!()
            }
            let passes =
                |case: &str| println!("\n================ {} passes ================\n", case);
            use crate::test_case::TestCase;
            use assertion::Assertion;
            use ids::{CardId, NodeId};
            use serde_dson::from_dson;
            use std::sync::Arc;
            let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
            let input = std::fs::read_to_string(format!(
                "{}/cases/{}.dson",
                manifest_dir,
                stringify!($case).get(4..).unwrap()
            ))
            .unwrap();
            use deskc::card::CardQueries;
            use deskc::{Code, SyntaxKind};
            let mut compiler = deskc::card::CardsCompiler::default();
            let card_id = CardId::new();
            compiler.set_code(
                card_id.clone(),
                Code::SourceCode {
                    syntax: SyntaxKind::Minimalist,
                    source: Arc::new(input.clone()),
                },
            );
            let thir = compiler
                .thir(card_id)
                .unwrap_or_else(|err| print_errors(&input, err));
            let dson = thir2dson::thir_to_dson(&thir).unwrap();
            let test_case: TestCase = from_dson(dson).unwrap();
            // compile sources
            let input = &test_case.files[0].content;

            let card_id = CardId::new();
            compiler.set_code(
                card_id.clone(),
                Code::SourceCode {
                    syntax: SyntaxKind::Minimalist,
                    source: Arc::new(input.to_string()),
                },
            );

            for assertion in test_case.assertions.iter() {
                match assertion {
                    Assertion::Typed(typings) => {
                        use dson::Dson;
                        use hir::expr::Expr;
                        use hir::helper::HirVisitor;
                        use hir::meta::WithMeta;
                        use std::collections::HashMap;
                        #[derive(Default)]
                        struct HirIds {
                            ids: Vec<(usize, NodeId)>,
                        }
                        impl HirVisitor for HirIds {
                            fn visit_expr(&mut self, expr: &WithMeta<Expr>) {
                                if let Some(Dson::Literal(dson::Literal::Integer(int))) =
                                    expr.meta.attrs.first()
                                {
                                    self.ids.push((*int as usize, expr.id.clone()));
                                }
                                self.super_visit_expr(expr);
                            }
                        }
                        let mut hir_ids = HirIds::default();
                        let hir_result = compiler
                            .hir(card_id.clone())
                            .unwrap_or_else(|err| print_errors(input, err));
                        hir_ids.visit_expr(&hir_result.hir);
                        let attrs = hir_ids.ids.into_iter().collect::<HashMap<_, _>>();

                        let ctx = compiler
                            .typeinfer(card_id.clone())
                            .unwrap_or_else(|err| print_errors(input, err));
                        let types = ctx.get_types();

                        for (id, ty) in typings {
                            let actual =
                                attrs.get(id).and_then(|id| types.get(id).cloned()).unwrap();
                            assert_eq!(actual, *ty);
                        }
                        passes("Typed");
                    }
                    _ => {}
                }
            }

            let mir = compiler
                .mir(card_id)
                .unwrap_or_else(|err| print_errors(input, err));
            let mut miri = miri::eval_mir((*mir).clone());
            use dprocess::interpreter::Interpreter;
            let start = std::time::Instant::now();
            let value = loop {
                match miri.reduce(&std::time::Duration::from_secs(1)).unwrap() {
                    dprocess::interpreter_output::InterpreterOutput::Returned(ret) => break ret,
                    dprocess::interpreter_output::InterpreterOutput::Performed {
                        input,
                        effect,
                    } => {
                        panic!("perform {:?} {:?}", input, effect)
                    }
                    dprocess::interpreter_output::InterpreterOutput::Running => continue,
                }
            };
            let end = std::time::Instant::now();
            println!("{:?}", end - start);
            for assertion in test_case.assertions.iter() {
                match assertion {
                    Assertion::RunSuccess { result } => {
                        assert_eq!(value, *result);
                        passes("RunSuccess");
                    }
                    _ => {}
                }
            }
        }
    };
}

test!(case001_literal);
test!(case002_addition);
test!(case003_match);
test!(case004_let_function);
// test!(case005);
test!(case006_continuation);
test!(case007_fibonacci);
// test!(case008);
