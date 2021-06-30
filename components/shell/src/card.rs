use bevy_math::Vec2;
use language::code::node::{Node, NodeData};
use physics::widget::{component::sugar as c, Target, Widget};
use protocol::card_id::CardId;

pub struct Card {
    pub id: CardId,
}

impl Card {
    pub fn new() -> Self {
        Self { id: CardId::new() }
    }
}

/// A struct for a computed value with its type and encoding.
pub struct Computed(pub Node);

pub fn render_card(
    card: &Card,
    node: &Node,
    computed: Option<&Computed>,
    position: Vec2,
) -> Option<Widget> {
    use NodeData::*;
    let id = card.id.to_string().into();
    match &node.data {
        Literal { value } => match value {
            language::code::node::LiteralValue::Unit => {
                return Some(Widget {
                    id,
                    position,
                    shape: None,
                    component: c::blank(),
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
                    component: c::input_string("TODO", value),
                })
            }
            language::code::node::LiteralValue::Number(value) => {
                return Some(Widget {
                    id,
                    position,
                    shape: None,
                    component: c::input_number("TODO", value.to_owned()),
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
