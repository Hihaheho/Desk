mod add;

use crate::type_::Type;

use super::{BinaryArithmeticOperator, BinaryOperator, Code, CodeData, LiteralValue};

pub(crate) fn reduce(operator: BinaryOperator, operands: (&Code, &Code)) -> Code {
    use BinaryOperator::*;
    match operator {
        Arithmetic(arithmetic_operator) => {
            use BinaryArithmeticOperator::*;
            match arithmetic_operator {
                Add => {
                    match (
                        super::reduce::reduce(operands.0),
                        super::reduce::reduce(operands.1),
                    ) {
                        (
                            Code {
                                data:
                                    CodeData::Literal {
                                        value: LiteralValue::Number(left),
                                    },
                                type_: _,
                                metadata: _,
                            },
                            Code {
                                data:
                                    CodeData::Literal {
                                        value: LiteralValue::Number(right),
                                    },
                                type_: _,
                                metadata: _,
                            },
                        ) => Code {
                            data: CodeData::Literal {
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
    use crate::code::node::sugar;

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
