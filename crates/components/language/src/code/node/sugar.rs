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

pub fn integer(value: i32) -> Node {
    node(
        NodeData::Literal {
            value: LiteralValue::Number(NumberLiteral::Integer(value)),
        },
        Type::Number,
    )
}

pub fn string(value: impl Into<String>) -> Node {
    node(
        NodeData::Literal {
            value: LiteralValue::String(value.into()),
        },
        Type::String,
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
