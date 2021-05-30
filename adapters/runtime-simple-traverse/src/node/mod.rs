use language::abstract_syntax_tree::node::{Node, NodeData};

pub fn reduce(node: &Node) -> &Node {
    use NodeData::*;
    match &node.data {
        Literal { value } => node,
        Variable { identifier } => {
            todo!()
        }
        ApplyUnaryOperator { operator, operand } => {
            todo!()
        }
        ApplyBinaryOperator { operator, operands } => {
            todo!()
        }
        ApplyFunction { function, argument } => {
            todo!()
        }
        Function {
            parameter,
            expression,
        } => {
            todo!()
        }
        Perform { effect, argument } => {
            todo!()
        }
        Handle {
            expression,
            acc,
            handlers,
        } => {
            todo!()
        }
        Let {
            variable,
            value,
            expression,
        } => {
            todo!()
        }
    }
}
