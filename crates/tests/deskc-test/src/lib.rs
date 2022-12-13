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
                } else if let Some(mirgen_error) =
                    error.downcast_ref::<errors::mirgen::GenMirError>()
                {
                    mirgen_error.into()
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
            use crate::test_case::{RunResult, TestCase};
            use ids::{Entrypoint, FileId, NodeId};
            use serde_dson::from_dson;
            use std::sync::Arc;

            // load test case
            let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
            let case_file = format!(
                "{}/cases/{}.dson",
                manifest_dir,
                stringify!($case).get(4..).unwrap()
            );
            let input = std::fs::read_to_string(&case_file)
                .expect(&format!("case file not found {}", case_file));
            use deskc::card::{DeskCompiler, DeskcQueries};
            use deskc::{Code, SyntaxKind};
            let mut compiler = DeskCompiler::default();
            let case_file_id = FileId::new();
            compiler.set_code(
                case_file_id.clone(),
                Code::SourceCode {
                    syntax: SyntaxKind::Minimalist,
                    source: Arc::new(input.clone()),
                },
            );
            // Type check of case file
            let _ = compiler
                .typeinfer(Entrypoint::File(case_file_id.clone()))
                .unwrap_or_else(|err| print_errors(&input, err));
            let ast = compiler
                .ast(case_file_id)
                .unwrap_or_else(|err| print_errors(&input, err));
            let dson = ast.as_ref().clone().try_into().unwrap_or_else(|err| {
                panic!("failed to convert ast to dson: {:?}", err);
            });
            let test_case: TestCase = from_dson(dson).unwrap();

            // assertions
            for file in test_case.files {
                compiler.set_code(
                    file.id,
                    Code::SourceCode {
                        syntax: SyntaxKind::Minimalist,
                        source: Arc::new(file.content),
                    },
                );
            }
            let input = |compiler: &DeskCompiler, entrypoint: &Entrypoint| match compiler
                .code(entrypoint.file_id().clone())
            {
                Code::SourceCode { source, .. } => source,
                _ => panic!("cannot get source code"),
            };

            if let Some(typed_vec) = test_case.assertions.typed {
                use dson::Dson;
                use hir::expr::Expr;
                use hir::meta::WithMeta;
                use hir::visitor::HirVisitor;
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
                            self.ids.push((*int as usize, expr.meta.id.clone()));
                        }
                        self.super_visit_expr(expr);
                    }
                }

                for typed in typed_vec {
                    let mut hir_ids = HirIds::default();
                    let hir = compiler
                        .hir(typed.entrypoint.clone())
                        .unwrap_or_else(|err| {
                            print_errors(&input(&compiler, &typed.entrypoint), err)
                        });
                    hir_ids.visit_expr(&hir);
                    let attrs = hir_ids.ids.into_iter().collect::<HashMap<_, _>>();

                    let conclusions =
                        compiler
                            .typeinfer(typed.entrypoint.clone())
                            .unwrap_or_else(|err| {
                                print_errors(&input(&compiler, &typed.entrypoint), err)
                            });

                    for (id, ty) in typed.typings {
                        let actual = attrs
                            .get(&id)
                            .and_then(|id| conclusions.get_type(id).cloned())
                            .expect(&format!("no type for {}", id));
                        assert_eq!(actual, ty, "type mismatch for {}", id);
                    }
                }
                passes("Typed");
            }

            if let Some(runs) = test_case.assertions.runs {
                for run in runs {
                    let conclusion =
                        compiler
                            .typeinfer(run.entrypoint.clone())
                            .unwrap_or_else(|err| {
                                print_errors(&input(&compiler, &run.entrypoint), err)
                            });
                    let mir = compiler.mir(run.entrypoint.clone()).unwrap_or_else(|err| {
                        print_errors(&input(&compiler, &run.entrypoint), err)
                    });
                    use dprocess::interpreter_builder::InterpreterBuilder;
                    let mut miri = miri::try_create_miri_builder(
                        (*mir).clone(),
                        &Default::default(),
                        conclusion,
                    )
                    .unwrap()
                    .build();
                    let start = std::time::Instant::now();
                    let value = loop {
                        match miri.reduce(&std::time::Duration::from_secs(1)).unwrap() {
                            dprocess::interpreter_output::InterpreterOutput::Returned(ret) => {
                                break ret
                            }
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
                    println!("elapsed {:?}", end - start);
                    match run.result {
                        RunResult::Success(result) => {
                            assert_eq!(value, result);
                        }
                    }
                }
                passes("RunSuccess");
            }
        }
    };
}

test!(case001_literal);
test!(case002_addition);
test!(case003_match);
test!(case004_let_function);
test!(case005_division_by_zero);
test!(case006_continuation);
test!(case007_fibonacci);
// link is not implemented yet
// test!(case008_cards);
