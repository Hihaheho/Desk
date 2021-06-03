mod create;

use bevy_math::Vec2;
pub use create::*;
use editor::{
    card::{Card, Computed},
    widget::{Component, Target, Widget},
};
use language::code::{
    node::{Node, NodeData},
    path::NodePath,
};

pub fn render_card(
    card: &Card,
    node: &Node,
    computed: Option<&Computed>,
    position: Vec2,
) -> Option<Widget> {
    use NodeData::*;
    let id = card.card_id.to_string();
    match &node.data {
        Literal { value } => match value {
            language::code::node::LiteralValue::Unit => {
                return Some(Widget {
                    id,
                    position,
                    shape: None,
                    component: Component::Unit,
                })
            }
            language::code::node::LiteralValue::Label(_) => {
                todo!()
            }
            language::code::node::LiteralValue::Bool(_) => {
                todo!()
            }
            language::code::node::LiteralValue::String(value) => {
                return Some(Widget {
                    id,
                    position,
                    shape: None,
                    component: Component::InputString {
                        value: value.to_owned(),
                        target: Target {
                            card_id: card.card_id,
                            node_path: NodePath::new(vec![]),
                        },
                    },
                })
            }
            language::code::node::LiteralValue::Number(value) => {
                return Some(Widget {
                    id,
                    position,
                    shape: None,
                    component: Component::InputNumber {
                        value: value.to_owned(),
                        target: Target {
                            card_id: card.card_id,
                            node_path: NodePath::new(vec![]),
                        },
                    },
                })
            }
            language::code::node::LiteralValue::Array(_) => {
                todo!()
            }
            language::code::node::LiteralValue::Product(_) => {
                todo!()
            }
            language::code::node::LiteralValue::Sum(_) => {
                todo!()
            }
            language::code::node::LiteralValue::Type(_) => {
                todo!()
            }
        },
        Let {
            variable,
            value,
            expression,
        } => {
            todo!()
        }
        Variable { identifier } => {
            todo!()
        }
        ApplyUnaryOperator { operator, operand } => {
            todo!()
        }
        ApplyBinaryOperator { operator, operands } => {
            // TODO
        }
        Function {
            parameter,
            expression,
        } => {
            todo!()
        }
        ApplyFunction { function, argument } => {
            todo!()
        }
        Perform { effect, argument } => {
            todo!()
        }
        Handle {
            expression,
            acc,
            handlers,
        } => {
            todo!()
        }
    }
    None
}
