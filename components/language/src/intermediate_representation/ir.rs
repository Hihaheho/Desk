use protocol::id::Id;

use crate::{
    abstract_syntax_tree::node::{Identifier, LiteralValue},
    type_::Type,
};

#[derive(Hash, Eq, PartialEq)]
pub struct OperatorId(pub Id);

/// High level intermediate representation
pub struct IR {
    pub node: IRNode,
    pub return_type: Type,
}

pub struct Function {
    pub parameter: Identifier,
    pub expression: Box<IR>,
}

pub enum IRNode {
    Literal {
        literal_value: LiteralValue,
    },
    Variable {
        identifier: Identifier,
    },
    Function(Function),
    Apply {
        function: Box<IR>,
        argument: Box<IR>,
    },
    Operate {
        operator_id: OperatorId,
        operands: Vec<IR>,
    },
    Perform {
        effect: Box<IR>,
        argument: Box<IR>,
    },
    Handle {
        expression: Box<IR>,
        effect: Type,
        effect_parameter: Identifier,
        continuation: Identifier,
        handler: Box<IR>,
    },
}
