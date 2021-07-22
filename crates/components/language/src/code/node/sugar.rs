use crate::type_::Type;

use super::{
    BinaryArithmeticOperator, BinaryOperator, Code, CodeData, LiteralValue, NumberLiteral,
};

pub fn node(data: CodeData, type_: Type) -> Code {
    Code {
        data,
        type_,
        metadata: None,
    }
}

pub fn integer(value: i32) -> Code {
    node(
        CodeData::Literal {
            value: LiteralValue::Number(NumberLiteral::Integer(value)),
        },
        Type::Number,
    )
}

pub fn string(value: impl Into<String>) -> Code {
    node(
        CodeData::Literal {
            value: LiteralValue::String(value.into()),
        },
        Type::String,
    )
}

pub fn add(left: Code, right: Code) -> Code {
    node(
        CodeData::ApplyBinaryOperator {
            operator: BinaryOperator::Arithmetic(BinaryArithmeticOperator::Add),
            operands: (Box::new(left), Box::new(right)),
        },
        Type::Number,
    )
}
