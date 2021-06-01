mod create;

pub use create::*;
use editor::{
    card::{Card, Computed},
    widget::{Target, Widget},
};
use language::abstract_syntax_tree::{
    node::{Node, NodeData},
    operation::NodeOperation,
    path::NodePath,
};

pub fn render_card(card: &Card, node: &Node, computed: Option<&Computed>) -> Option<Widget> {
    use NodeData::*;
    match &node.data {
        Literal { value } => match value {
            language::abstract_syntax_tree::node::LiteralValue::Unit => return Some(Widget::Unit),
            language::abstract_syntax_tree::node::LiteralValue::Label(_) => {
                todo!()
            }
            language::abstract_syntax_tree::node::LiteralValue::Bool(_) => {
                todo!()
            }
            language::abstract_syntax_tree::node::LiteralValue::String(value) => {
                return Some(Widget::InputString {
                    value: value.to_owned(),
                    target: Target {
                        card_id: card.card_id,
                        node_path: NodePath::new(vec![]),
                    },
                })
            }
            language::abstract_syntax_tree::node::LiteralValue::Number(value) => {
                return Some(Widget::InputNumber {
                    value: value.to_owned(),
                    target: Target {
                        card_id: card.card_id,
                        node_path: NodePath::new(vec![]),
                    },
                })
            }
            language::abstract_syntax_tree::node::LiteralValue::Array(_) => {
                todo!()
            }
            language::abstract_syntax_tree::node::LiteralValue::Product(_) => {
                todo!()
            }
            language::abstract_syntax_tree::node::LiteralValue::Sum(_) => {
                todo!()
            }
            language::abstract_syntax_tree::node::LiteralValue::Type(_) => {
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
