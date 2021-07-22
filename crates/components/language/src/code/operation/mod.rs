use super::node::{Code, CodeData, LiteralValue, NumberLiteral};

#[derive(Clone, Debug, PartialEq)]
pub struct CodeOperations(pub Vec<CodeOperation>);

impl CodeOperations {
    pub fn iter(&self) -> impl Iterator<Item = &CodeOperation> {
        self.0.iter()
    }
}

impl From<Vec<CodeOperation>> for CodeOperations {
    fn from(from: Vec<CodeOperation>) -> Self {
        Self(from)
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub enum CodeOperation {
    UpdateString(String),
    UpdateNumber(NumberLiteral),
}

use CodeOperation::*;

#[derive(Clone, Debug, PartialEq)]
pub struct CodeOperationError {
    pub node: Code,
    pub operation: CodeOperation,
}

impl Code {
    pub fn apply_operation(
        &self,
        node_operation: &CodeOperation,
    ) -> Result<Self, CodeOperationError> {
        match (&self.data, node_operation) {
            (
                super::node::CodeData::Literal {
                    value: LiteralValue::String(_),
                },
                UpdateString(new_value),
            ) => Ok(Self {
                data: CodeData::Literal {
                    value: LiteralValue::String(new_value.to_owned()),
                },
                ..self.to_owned()
            }),
            (_, UpdateString(_)) => Err(CodeOperationError {
                node: self.clone(),
                operation: node_operation.to_owned(),
            }),
            (
                super::node::CodeData::Literal {
                    value: LiteralValue::Number(_),
                },
                UpdateNumber(new_value),
            ) => Ok(Self {
                data: CodeData::Literal {
                    value: LiteralValue::Number(new_value.to_owned()),
                },
                ..self.to_owned()
            }),
            (_, UpdateNumber(_)) => Err(CodeOperationError {
                node: self.clone(),
                operation: node_operation.to_owned(),
            }),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::code::node::sugar;

    use super::*;

    #[test]
    fn update_string() {
        assert_eq!(
            sugar::string("a").apply_operation(&UpdateString("b".to_string())),
            Ok(sugar::string("b"))
        );
    }

    #[test]
    fn update_number() {
        assert_eq!(
            sugar::integer(1).apply_operation(&UpdateNumber(NumberLiteral::Integer(2))),
            Ok(sugar::integer(2))
        );
    }
}
