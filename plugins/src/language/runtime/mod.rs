use std::collections::HashMap;

use bevy::prelude::*;
use language::{
    semantic::ir::{OperatorId, IR},
    Operator,
};
use protocol::id::create_consistent_id;
use runtime::{definition::OperatorDefinitions, ComputedValue, EncodedValue};

pub struct RuntimePlugin;

struct PlusOperator;

impl Operator for PlusOperator {
    // fn operate(&self, operands: Vec<ComputedValue>) -> ComputedValue {
    //     match (&operands[0].encoded_value, &operands[1].encoded_value) {
    //         (EncodedValue::I32(left), EncodedValue::I32(right)) => ComputedValue {
    //             type_: operands[0].type_.to_owned(),
    //             encoded_value: EncodedValue::I32(left + right),
    //         },
    //         _ => todo!(),
    //     }
    // }
}

impl Plugin for RuntimePlugin {
    fn build(&self, app: &mut AppBuilder) {
        let mut operator_definitions = OperatorDefinitions {
            map: HashMap::new(),
        };
        operator_definitions.map.insert(
            OperatorId(create_consistent_id("desk-plugins", "plus")),
            Box::new(PlusOperator),
        );
        app.add_system(run.system())
            .insert_resource(operator_definitions);
    }
}

fn run(
    mut commands: Commands,
    operator_definitions: Res<OperatorDefinitions>,
    query: Query<(Entity, &IR), Changed<IR>>,
) {
    for (entity, code) in query.iter() {
        commands
            .entity(entity)
            .insert(prototype::compute_on_stack(&operator_definitions, code));
    }
}

mod prototype {
    use language::syntax::ast::LiteralValue;
    use runtime::EncodedValue;

    use super::*;
    pub fn compute_on_stack(
        operator_definitions: &OperatorDefinitions,
        code: &IR,
    ) -> ComputedValue {
        use EncodedValue::*;
        let IR { node, return_type } = code;
        let encoded_value = match node {
            language::semantic::ir::IRNode::Literal { literal_value } => match literal_value {
                LiteralValue::String(value) => String(value.to_owned()),
                LiteralValue::I32(value) => I32(*value),
                LiteralValue::F32(value) => F32(*value),
            },
            language::semantic::ir::IRNode::Operate {
                operator_id,
                operands,
            } => {
                todo!()
                // operator_definitions
                //     .map
                //     .get(operator_id)
                //     .unwrap()
                // .operate(
                //     operands
                //         .iter()
                //         .map(|operand| compute_on_stack(operator_definitions, operand))
                //         .collect(),
                // )                // .encoded_value
            }
            language::semantic::ir::IRNode::Variable { identifier } => {
                todo!()
            }
            language::semantic::ir::IRNode::Function {
                parameter,
                expression,
            } => {
                todo!()
            }
            language::semantic::ir::IRNode::Apply { function, argument } => {
                todo!()
            }
            language::semantic::ir::IRNode::Perform { effect, argument } => {
                todo!()
            }
            language::semantic::ir::IRNode::Handle {
                expression,
                effect,
                effect_parameter,
                continuation,
                handler,
            } => {
                todo!()
            }
        };
        ComputedValue {
            type_: return_type.to_owned(),
            encoded_value: encoded_value,
        }
    }
}
