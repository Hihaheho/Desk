use language::abstract_syntax_tree::node::Node;
use runtime::Runtime;

pub struct SimpleTraverseRuntime;

impl Runtime for SimpleTraverseRuntime {
    type Code = Node;
    type Error = ();

    fn generate_code(&self, ir: &Node) -> Self::Code {
        ir.clone()
    }

    fn run(&self, code: &Node) -> Result<Node, Self::Error> {
        // TODO reduce while reducible.
        Ok(code.reduce())
    }
}
