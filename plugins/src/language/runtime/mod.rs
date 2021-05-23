use std::collections::HashMap;

use bevy::prelude::*;
use language::{intermediate_representation::ir::IR, Operator};
use protocol::id::create_consistent_id;

pub struct RuntimePlugin;

impl Plugin for RuntimePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(run.system());
    }
}

fn run(mut commands: Commands, query: Query<(Entity, &IR), Changed<IR>>) {
    for (entity, code) in query.iter() {
        commands
            .entity(entity)
            .insert(prototype::compute_on_stack(code));
    }
}

mod prototype {
    use language::abstract_syntax_tree::node::LiteralValue;
    use runtime::{ComputedValue, EncodedValue};

    use super::*;
    pub fn compute_on_stack(code: &IR) -> ComputedValue {
        use EncodedValue::*;
        let IR { node, return_type } = code;
        let encoded_value = match node {
            language::intermediate_representation::ir::IRNode::Literal { literal_value } => {
                match literal_value {
                    LiteralValue::String(value) => String(value.to_owned()),
                    _ => todo!(),
                }
            }
            language::intermediate_representation::ir::IRNode::Operate {
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
            language::intermediate_representation::ir::IRNode::Variable { identifier } => {
                todo!()
            }
            language::intermediate_representation::ir::IRNode::Function(function) => {
                todo!()
            }
            language::intermediate_representation::ir::IRNode::Apply { function, argument } => {
                todo!()
            }
            language::intermediate_representation::ir::IRNode::Perform { effect, argument } => {
                todo!()
            }
            language::intermediate_representation::ir::IRNode::Handle {
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
