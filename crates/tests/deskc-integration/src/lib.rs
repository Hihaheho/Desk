mod assertion;
mod test_case;

macro_rules! test {
    ($case:ident, $file_name:expr) => {
        #[test]
        fn $case() {
            let passes =
                |case: &str| println!("\n================ {} passes ================\n", case);
            use crate::test_case::TestCase;
            use assertion::Assertion;
            use serde_dson::from_dson;
            let input = include_str!($file_name);
            let tokens = lexer::scan(input).unwrap();
            let ast = parser::parse(tokens).unwrap();
            let (genhir, hir) = hirgen::gen_hir(&ast).unwrap();
            let (ctx, _ty) = typeinfer::synth(genhir.next_id(), &hir).unwrap();
            let thir = thirgen::gen_typed_hir(ctx.next_id(), ctx.get_types(), &hir);
            let dson = thir2dson::thir_to_dson(&thir).unwrap();
            let test_case: TestCase = from_dson(dson).unwrap();
            // compile sources
            let input = &test_case.files[0].content;
            fn print_errors<T>(
                input: &str,
                diagnostics: impl Into<textual_diagnostics::TextualDiagnostics>,
            ) -> T {
                use ariadne::{Label, Report, ReportKind, Source};
                let diagnostics = diagnostics.into();
                let report =
                    Report::build(ReportKind::Error, (), 0).with_message(diagnostics.title);
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
            let tokens = match lexer::scan(input) {
                Ok(tokens) => tokens,
                Err(errors) => print_errors(input, errors),
            };
            let ast = match parser::parse(tokens) {
                Ok(ast) => ast,
                Err(errors) => print_errors(input, errors),
            };
            let (genhir, hir) = hirgen::gen_cards(&ast).unwrap();
            let entrypoint = hir.entrypoint.unwrap();
            let ctx = match typeinfer::synth(genhir.next_id(), &entrypoint) {
                Ok((ctx, _ty)) => ctx,
                Err(error) => print_errors(input, error),
            };

            for assertion in test_case.assertions.iter() {
                match assertion {
                    Assertion::Typed(typings) => {
                        use std::collections::HashMap;
                        let attrs: HashMap<String, ids::NodeId> = genhir
                            .attrs
                            .borrow()
                            .iter()
                            .flat_map(|(id, attrs)| {
                                attrs.iter().map(|attr| (format!("{:?}", attr), id.clone()))
                            })
                            .collect();
                        let types = ctx.get_types();
                        for (id, ty) in typings {
                            let actual = attrs
                                .get(&format!(
                                    "{:?}",
                                    hir::expr::Expr::Literal(hir::expr::Literal::Integer(
                                        *id as i64
                                    ))
                                ))
                                .and_then(|id| types.get(id).cloned())
                                .unwrap();
                            assert_eq!(actual, *ty);
                        }
                        passes("Typed");
                    }
                    _ => {}
                }
            }

            let thir = thirgen::gen_typed_hir(ctx.next_id(), ctx.get_types(), &entrypoint);
            let amirs = amirgen::gen_abstract_mir(&thir).unwrap();
            let mirs = concretizer::concretize(&amirs);
            let mut miri = miri::eval_mirs(mirs);
            let value = loop {
                match miri.eval_next() {
                    miri::Output::Return(ret) => break ret,
                    miri::Output::Perform { input, effect } => {
                        panic!("perform {:?} {:?}", input, effect)
                    }
                    miri::Output::Running => continue,
                }
            };
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
