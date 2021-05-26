use language::abstract_syntax_tree::node::{
    BinaryArithmeticOperator, BinaryOperator, LiteralValue, Node, NodeData,
};

pub fn reduce(operator: BinaryOperator, operands: (Node, Node)) -> Node {
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
                                value: LiteralValue::Number(left + right),
                            },
                            type_: None,
                            metadata: 0,
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
    use language::abstract_syntax_tree::node::{
        BinaryArithmeticOperator, LiteralValue, NodeData, NumberLiteral,
    };

    use super::*;

    #[test]
    fn add_numbers() {
        let one = Node {
            data: NodeData::Literal {
                value: LiteralValue::Number(NumberLiteral::Integer(1)),
            },
            type_: None,
            metadata: 0,
        };
        let two = Node {
            data: NodeData::Literal {
                value: LiteralValue::Number(NumberLiteral::Integer(2)),
            },
            type_: None,
            metadata: 0,
        };
        let three = Node {
            data: NodeData::Literal {
                value: LiteralValue::Number(NumberLiteral::Integer(3)),
            },
            type_: None,
            metadata: 0,
        };
        assert_eq!(
            reduce(
                BinaryOperator::Arithmetic(BinaryArithmeticOperator::Add),
                (one, two)
            ),
            three
        );
    }
}
