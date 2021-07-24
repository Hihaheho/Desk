use language::{code::node::Code, Runtime, RuntimeError};

pub struct SimpleTraverseRuntime;

impl Runtime for SimpleTraverseRuntime {
    fn run(&self, code: &Code) -> Result<Code, RuntimeError> {
        // TODO reduce while reducible.
        Ok(code.reduce())
    }
}
