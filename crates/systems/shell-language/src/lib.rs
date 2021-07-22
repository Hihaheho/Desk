mod event_handler;
mod literal_value;

use language::code::node::{Code, CodeData};
use physics::widget::component::{sugar as c, Component};

pub use event_handler::*;

use crate::literal_value::render_literal_number;

pub fn render_node(node: &Code) -> Component {
    use CodeData::*;
    match &node.data {
        Literal { value } => match value {
            language::code::node::LiteralValue::Unit => c::blank(),
            language::code::node::LiteralValue::Label(_) => {
                todo!()
            }
            language::code::node::LiteralValue::Bool(_) => {
                todo!()
            }
            language::code::node::LiteralValue::String(value) => c::input_string("TODO", value),
            language::code::node::LiteralValue::Number(value) => render_literal_number(value),
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
            c::blank()
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
}
