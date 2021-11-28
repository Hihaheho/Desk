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
            use file::FileId;
            use serde_dson::from_dson;
            let input = include_str!($file_name);
            let tokens = lexer::scan(input).unwrap();
            let ast = parser::parse(tokens).unwrap();
            let (genhir, hir) = hirgen::gen_hir(FileId(0), &ast, Default::default()).unwrap();
            let (ctx, _ty) = typeinfer::synth(genhir.next_id(), &hir).unwrap();
            let thir = thirgen::gen_typed_hir(ctx.next_id(), ctx.get_types(), &hir);
            let dson = thir2dson::thir_to_dson(&thir).unwrap();
            let test_case: TestCase = from_dson(dson).unwrap();
            // compile sources
            let input = &test_case.files[0].content;
            let tokens = lexer::scan(input).unwrap();
            let ast = parser::parse(tokens).unwrap();
            let (genhir, hir) = hirgen::gen_hir(FileId(0), &ast, Default::default()).unwrap();
            let (ctx, _ty) = typeinfer::synth(genhir.next_id(), &hir).unwrap();

            for assertion in test_case.assertions.iter() {
                match assertion {
                    Assertion::Typed(typings) => {
                        use std::collections::HashMap;
                        let attrs: HashMap<String, usize> = genhir
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
                                    hir::expr::Expr::Literal(hir::expr::Literal::Int(*id as i64))
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

            let thir = dbg!(thirgen::gen_typed_hir(ctx.next_id(), ctx.get_types(), &hir));
            let amirs = dbg!(amirgen::gen_abstract_mir(&thir).unwrap());
            let mirs = dbg!(concretizer::concretize(&amirs));
            let mut evalmir = evalmir::eval_mirs(mirs);
            let value = loop {
                match evalmir.eval_next() {
                    evalmir::Output::Return(ret) => break ret,
                    evalmir::Output::Perform { input, effect } => panic!("perform {:?} {:?}", input, effect),
                    evalmir::Output::Running => continue,
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
test!(case005, "../cases/005_division_by_zero.dson");
