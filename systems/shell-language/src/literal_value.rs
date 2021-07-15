use language::code::node::NumberLiteral;
use physics::widget::component::{sugar as c, Component};

pub fn render_literal_number(number: &NumberLiteral) -> Component {
    use NumberLiteral::*;
    match number {
        Integer(value) => c::input_integer("TODO", *value),
        _ => todo!(),
    }
}
