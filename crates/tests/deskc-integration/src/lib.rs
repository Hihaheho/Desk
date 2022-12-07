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
            use crate::test_case::{Entrypoint, RunResult, TestCase};
            use ids::{CardId, NodeId};
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
            use deskc::card::CardQueries;
            use deskc::{Code, SyntaxKind};
            let mut compiler = deskc::card::CardsCompiler::default();
            let case_card_id = CardId::new();
            compiler.set_code(
                case_card_id.clone(),
                Code::SourceCode {
                    syntax: SyntaxKind::Minimalist,
                    source: Arc::new(input.clone()),
                },
            );
            let thir = compiler
                .thir(case_card_id)
                .unwrap_or_else(|err| print_errors(&input, err));
            let dson = thir2dson::thir_to_dson(&thir).unwrap();
            let test_case: TestCase = from_dson(dson).unwrap();

            // assertions
            let mut file_to_card = std::collections::HashMap::new();
            let mut file_contents = std::collections::HashMap::new();
            for file in test_case.files {
                let card_id = CardId::new();
                file_to_card.insert(file.name.clone(), card_id.clone());
                file_contents.insert(file.name.clone(), file.content.clone());
                compiler.set_code(
                    card_id.clone(),
                    Code::SourceCode {
                        syntax: SyntaxKind::Minimalist,
                        source: Arc::new(file.content),
                    },
                );
            }
            let card_id = |file_name: &String| {
                file_to_card
                    .get(file_name)
                    .expect(&format!("file not found: {}", file_name))
                    .clone()
            };
            let input = |file_name: &String| {
                file_contents
                    .get(file_name)
                    .expect(&format!("file not found: {}", file_name))
            };

            if let Some(typed_vec) = test_case.assertions.typed {
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

                for typed in typed_vec {
                    let mut hir_ids = HirIds::default();
                    let hir_result = compiler
                        .hir(card_id(&typed.file))
                        .unwrap_or_else(|err| print_errors(input(&typed.file), err));
                    hir_ids.visit_expr(&hir_result.hir);
                    let attrs = hir_ids.ids.into_iter().collect::<HashMap<_, _>>();

                    let ctx = compiler
                        .typeinfer(card_id(&typed.file))
                        .unwrap_or_else(|err| print_errors(input(&typed.file), err));
                    let types = ctx.get_types();

                    for (id, ty) in typed.typings {
                        let actual = attrs
                            .get(&id)
                            .and_then(|id| types.get(id).cloned())
                            .expect(&format!("no type for {}", id));
                        assert_eq!(actual, ty, "type mismatch for {}", id);
                    }
                }
                passes("Typed");
            }

            if let Some(runs) = test_case.assertions.runs {
                for run in runs {
                    let mir = match run.entrypoint {
                        Entrypoint::File(file_name) => compiler
                            .mir(card_id(&file_name))
                            .unwrap_or_else(|err| print_errors(input(&file_name), err)),
                        Entrypoint::Card(uuid) => {
                            todo!()
                        }
                    };
                    let mut miri = miri::eval_mir((*mir).clone());
                    use dprocess::interpreter::Interpreter;
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
test!(case008_cards);
