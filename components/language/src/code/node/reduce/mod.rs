use crate::code::node::apply_binary_operator;

use super::{Node, NodeData};

pub fn reduce(node: &Node) -> Node {
    use NodeData::*;
    match &node.data {
        Literal { value: _ } => node.to_owned(),
        Variable { identifier: _ } => {
            todo!()
        }
        ApplyUnaryOperator {
            operator: _,
            operand: _,
        } => {
            todo!()
        }
        ApplyBinaryOperator { operator, operands } => {
            apply_binary_operator::reduce(*operator, (operands.0.as_ref(), operands.1.as_ref()))
        }
        ApplyFunction {
            function: _,
            argument: _,
        } => {
            todo!()
        }
        Function {
            parameter: _,
            expression: _,
        } => {
            todo!()
        }
        Perform {
            effect: _,
            argument: _,
        } => {
            todo!()
        }
        Handle {
            expression: _,
            acc: _,
            handlers: _,
        } => {
            todo!()
        }
        Let {
            variable: _,
            value: _,
            expression: _,
        } => {
            todo!()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::code::node::sugar;

    #[test]
    fn literal() {
        let a = sugar::integer(1);
        assert_eq!(reduce(&a), a);
    }

    #[test]
    fn apply_binary_operator() {
        assert_eq!(
            reduce(&sugar::add(sugar::integer(1), sugar::integer(2))),
            sugar::integer(3)
        );
    }
}
