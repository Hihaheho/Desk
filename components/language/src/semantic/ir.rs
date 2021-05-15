use protocol::id::Id;

use crate::typing::r#type::Type;

pub struct OperatorId(Id);

pub struct Code {
    pub operator: OperatorId,
    pub operands: Vec<Code>,
    pub return_type: Type,
}
