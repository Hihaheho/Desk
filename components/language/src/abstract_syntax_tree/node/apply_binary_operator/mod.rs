mod add;

use crate::type_::Type;

use super::{sugar, BinaryArithmeticOperator, BinaryOperator, LiteralValue, Node, NodeData};
use add::*;

pub fn reduce(operator: BinaryOperator, operands: (&Node, &Node)) -> Node {
    use BinaryOperator::*;
    match operator {
        Arithmetic(arithmetic_operator) => {
            use BinaryArithmeticOperator::*;
            match arithmetic_operator {
                Add => {
                    match (
                        super::node::reduce(operands.0),
                        super::node::reduce(operands.1),
                    ) {
                        (
                            Node {
                                data:
                                    NodeData::Literal {
                                        value: LiteralValue::Number(left),
                                    },
                                type_: type_,
                                metadata: _,
                            },
                            Node {
                                data:
                                    NodeData::Literal {
                                        value: LiteralValue::Number(right),
                                    },
                                type_: _,
                                metadata: _,
                            },
                        ) => Node {
                            data: NodeData::Literal {
                                value: LiteralValue::Number(left.add(&right)),
                            },
                            type_: Type::Number,
                            metadata: None,
                        },
                        _ => todo!(),
                    }
                }
                _ => todo!(),
            }
        }
        _ => todo!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_numbers() {
        assert_eq!(
            reduce(
                BinaryOperator::Arithmetic(BinaryArithmeticOperator::Add),
                (&sugar::integer(1), &sugar::integer(2))
            ),
            sugar::integer(3)
        );
    }
}
