mod apply_binary_operator;
mod apply_function;
mod apply_unary_operator;
mod handle;
mod let_;
mod node;
mod perform;

use language::abstract_syntax_tree::node::{Node, NodeData};
use runtime::Runtime;

use runtime::ComputedValue;

struct SimpleTraverseRuntime {}

impl Runtime for SimpleTraverseRuntime {
    type Code = Node;
    type Error = ();

    fn generate_code(ir: Node) -> Self::Code {
        ir.clone()
    }

    fn run(code: Self::Code) -> Result<runtime::ComputedValue, Self::Error> {
        let Node {
            data,
            type_,
            metadata,
        } = node::reduce(code);

        Ok(ComputedValue {
            type_: type_.unwrap(),
            encoded_value: todo!(),
        })
    }
}
