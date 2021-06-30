use language::code::node::NumberLiteral;

use super::{Component, InputId};

pub fn blank() -> Component {
    Component::Blank
}

pub fn label<T: Into<String>>(label: T) -> Component {
    Component::Label(label.into())
}

pub fn vertical_array(items: Vec<Component>) -> Component {
    Component::Array {
        orientation: super::Orientation::Vertical,
        items: items,
    }
}

pub fn horizontal_array(items: Vec<Component>) -> Component {
    Component::Array {
        orientation: super::Orientation::Horizontal,
        items: items,
    }
}

pub fn input_string<I: Into<InputId>, T: Into<String>>(id: I, default: T) -> Component {
    Component::InputString {
        id: id.into(),
        value: default.into(),
    }
}

pub fn input_number<T: Into<InputId>>(id: T, default: NumberLiteral) -> Component {
    Component::InputNumber {
        id: id.into(),
        value: default,
    }
}
