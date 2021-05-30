mod apply_binary_operator;
mod apply_function;
mod apply_unary_operator;
mod handle;
mod let_;
mod node;
mod perform;

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
        Ok(node::reduce(code).to_owned())
    }
}
