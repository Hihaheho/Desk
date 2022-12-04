mod assertion;
mod test_case;

macro_rules! test {
    ($case:ident, $file_name:expr) => {
        #[test]
        fn $case() {
            fn print_errors<T>(
                input: &str,
                error: deskc::query_result::QueryError,
            ) -> T {
                use ariadne::{Label, Report, ReportKind, Source};
                use errors::textual_diagnostics::{Report as TDReport, TextualDiagnostics};
                let diagnostics: TextualDiagnostics = if let Some(syntax_error) = error.downcast_ref::<errors::syntax::SyntaxError>() {
                    syntax_error.into()
                } else if let Some(typeinfer_error) = error.downcast_ref::<errors::typeinfer::ExprTypeError>() {
                    typeinfer_error.into()
                } else {
                    panic!("unexpected error: {:?}", error);
                };
                let report =
                    Report::build(ReportKind::Error, (), 0).with_message(diagnostics.title);
                diagnostics
                    .reports
                    .into_iter()
                    .fold(
                        report,
                        |report, TDReport { span, text }| {
                            report.with_label(Label::new(span).with_message(text))
                        },
                    )
                    .finish()
                    .print(Source::from(input))
                    .unwrap();
                panic!()
            }
            let passes =
                |case: &str| println!("\n================ {} passes ================\n", case);
            use ids::{CardId, NodeId};
            use std::sync::Arc;
            use crate::test_case::TestCase;
            use assertion::Assertion;
            use serde_dson::from_dson;
            let input = include_str!($file_name);
            use deskc::card::CardQueries;
            use deskc::{Code, SyntaxKind};
            let mut compiler = deskc::card::CardsCompiler::default();
            let card_id = CardId::new();
            compiler.set_code(
                card_id.clone(),
                Code::SourceCode {
                    syntax: SyntaxKind::Minimalist,
                    source: Arc::new(input.to_string()),
                },
            );
            let thir = compiler.thir(card_id).unwrap_or_else(|err| print_errors(input, err));
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
                        use std::collections::HashMap;
                        use hir::meta::WithMeta;
                        use hir::expr::Expr;
                        use hir::helper::HirVisitor;
                        use dson::Dson;
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
                        let hir_result = compiler.hir(card_id.clone()).unwrap_or_else(|err| print_errors(input, err));
                        hir_ids.visit_expr(&hir_result.hir);
                        let attrs = hir_ids.ids.into_iter().collect::<HashMap<_, _>>();

                        let ctx = compiler.typeinfer(card_id.clone()).unwrap_or_else(|err| print_errors(input, err));
                        let types = ctx.get_types();

                        for (id, ty) in typings {
                            let actual = attrs
                                .get(id)
                                .and_then(|id| types.get(id).cloned())
                                .unwrap();
                            assert_eq!(actual, *ty);
                        }
                        passes("Typed");
                    }
                    _ => {}
                }
            }

            let mir = compiler.mir(card_id).unwrap_or_else(|err| print_errors(input, err));
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

test!(case001, "../cases/001_literal.dson");
test!(case002, "../cases/002_addition.dson");
test!(case003, "../cases/003_match.dson");
test!(case004, "../cases/004_let_function.dson");
// test!(case005, "../cases/005_division_by_zero.dson");
test!(case006, "../cases/006_continuation.dson");
test!(case007, "../cases/007_fibonacci.dson");
// test!(case008, "../cases/008_cards.dson");
