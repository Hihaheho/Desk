use crate::type_::Type;

use super::{
    BinaryArithmeticOperator, BinaryOperator, LiteralValue, Node, NodeData, NumberLiteral,
};

pub fn node(data: NodeData, type_: Type) -> Node {
    Node {
        data,
        type_,
        metadata: None,
    }
}

pub fn integer(value: i64) -> Node {
    node(
        NodeData::Literal {
            value: LiteralValue::Number(NumberLiteral::Integer(value)),
        },
        Type::Number,
    )
}

pub fn add(left: Node, right: Node) -> Node {
    node(
        NodeData::ApplyBinaryOperator {
            operator: BinaryOperator::Arithmetic(BinaryArithmeticOperator::Add),
            operands: (Box::new(left), Box::new(right)),
        },
        Type::Number,
    )
}
