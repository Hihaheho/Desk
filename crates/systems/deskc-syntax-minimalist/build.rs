fn main() {
    parol::build::Builder::with_explicit_output_dir("src")
        .grammar_file("src/grammar.par")
        .expanded_grammar_output_file("grammar-exp.par")
        .parser_output_file("parser.rs")
        .actions_output_file("grammar_trait.rs")
        .enable_auto_generation()
        .max_lookahead(1)
        .unwrap()
        .generate_parser()
        .unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
