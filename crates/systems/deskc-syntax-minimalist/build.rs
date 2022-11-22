fn main() {
	parol::build::Builder::with_cargo_script_output()
	.grammar_file("src/grammar.par")
	.enable_auto_generation()
	.max_lookahead(1).unwrap()
	.generate_parser().unwrap();
	println!("cargo:rerun-if-changed=build.rs");
}
