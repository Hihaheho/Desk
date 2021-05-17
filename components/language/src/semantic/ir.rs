use protocol::id::Id;

use crate::{
    syntax::ast::{Identifier, LiteralValue},
    typing::type_::{EffectId, Type},
};

#[derive(Hash, Eq, PartialEq)]
pub struct OperatorId(pub Id);

/// High level intermediate representation
pub struct IR {
    pub node: IRNode,
    pub return_type: Type,
}

pub enum IRNode {
    Literal {
        literal_value: LiteralValue,
    },
    Variable {
        identifier: Identifier,
    },
    Function {
        parameter: Identifier,
        expression: Box<IR>,
    },
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
        effect: EffectId,
        effect_parameter: Identifier,
        continuation: Identifier,
        handler: Box<IR>,
    },
}
