use language::code::node::Code;
use runtime::Runtime;

pub struct SimpleTraverseRuntime;

impl Runtime for SimpleTraverseRuntime {
    type Code = Code;
    type Error = ();

    fn generate_code(&self, ir: &Code) -> Self::Code {
        ir.clone()
    }

    fn run(&self, code: &Code) -> Result<Code, Self::Error> {
        // TODO reduce while reducible.
        Ok(code.reduce())
    }
}
