use language::intermediate_representation::ir::IR;

pub trait CodeGenerator {
    type Code;

    fn generate(code: &IR) -> Self::Code;
}
