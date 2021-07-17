use super::node::{LiteralValue, Node, NodeData, NumberLiteral};

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub enum NodeOperation {
    UpdateString(String),
    UpdateNumber(NumberLiteral),
}

use NodeOperation::*;

#[derive(Clone, Debug, PartialEq)]
pub struct NodeOperationError {
    pub node: Node,
    pub operation: NodeOperation,
}

impl Node {
    pub fn apply_operation(
        &self,
        node_operation: &NodeOperation,
    ) -> Result<Self, NodeOperationError> {
        match (&self.data, node_operation) {
            (
                super::node::NodeData::Literal {
                    value: LiteralValue::String(_),
                },
                UpdateString(new_value),
            ) => Ok(Self {
                data: NodeData::Literal {
                    value: LiteralValue::String(new_value.to_owned()),
                },
                ..self.to_owned()
            }),
            (_, UpdateString(_)) => Err(NodeOperationError {
                node: self.clone(),
                operation: node_operation.to_owned(),
            }),
            (
                super::node::NodeData::Literal {
                    value: LiteralValue::Number(_),
                },
                UpdateNumber(new_value),
            ) => Ok(Self {
                data: NodeData::Literal {
                    value: LiteralValue::Number(new_value.to_owned()),
                },
                ..self.to_owned()
            }),
            (_, UpdateNumber(_)) => Err(NodeOperationError {
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
