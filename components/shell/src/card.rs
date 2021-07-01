use bevy_math::Vec2;
use language::code::node::{Node, NodeData};
use physics::widget::{component::sugar as c, Widget};
use protocol::card_id::CardId;

pub struct Card {
    pub id: CardId,
}

impl Card {
    pub fn new() -> Self {
        Self { id: CardId::new() }
    }
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

/// A struct for a computed value with its type and encoding.
pub struct Computed(pub Node);

pub fn render_card(
    card: &Card,
    node: &Node,
    _computed: Option<&Computed>,
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
            variable: _,
            value: _,
            expression: _,
        } => {
            todo!()
        }
        Variable { identifier: _ } => {
            todo!()
        }
        ApplyUnaryOperator {
            operator: _,
            operand: _,
        } => {
            todo!()
        }
        ApplyBinaryOperator {
            operator: _,
            operands: _,
        } => {
            // TODO
        }
        Function {
            parameter: _,
            expression: _,
        } => {
            todo!()
        }
        ApplyFunction {
            function: _,
            argument: _,
        } => {
            todo!()
        }
        Perform {
            effect: _,
            argument: _,
        } => {
            todo!()
        }
        Handle {
            expression: _,
            acc: _,
            handlers: _,
        } => {
            todo!()
        }
    }
    None
}
