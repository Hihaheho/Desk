pub mod card;

use language::code::node::Code;

pub trait Runtime {
    type Code;
    type Error;

    fn generate_code(&self, ir: &Code) -> Self::Code;
    fn run(&self, code: &Self::Code) -> Result<Code, Self::Error>;
}
