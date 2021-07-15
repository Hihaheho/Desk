pub mod card;

use language::code::node::Node;

pub trait Runtime {
    type Code;
    type Error;

    fn generate_code(&self, ir: &Node) -> Self::Code;
    fn run(&self, code: &Self::Code) -> Result<Node, Self::Error>;
}
