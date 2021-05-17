use language::semantic::ir::IR;

pub trait CodeGenerator {
    type Code;

    fn generate(code: &IR) -> Self::Code;
}
