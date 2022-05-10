use super::{Component, InputId, Integer};

pub fn blank() -> Component {
    Component::Blank
}

pub fn label<T: Into<String>>(label: T) -> Component {
    Component::Label(label.into())
}

pub fn vertical_array(items: Vec<Component>) -> Component {
    Component::Array {
        orientation: super::Orientation::Vertical,
        items,
    }
}

pub fn horizontal_array(items: Vec<Component>) -> Component {
    Component::Array {
        orientation: super::Orientation::Horizontal,
        items,
    }
}

pub fn input_string<I: Into<InputId>, T: Into<String>>(id: I, default: T) -> Component {
    Component::InputString {
        id: id.into(),
        value: default.into(),
    }
}

pub fn input_integer<T: Into<InputId>, I: Into<Integer>>(id: T, default: I) -> Component {
    Component::InputInteger {
        id: id.into(),
        value: default.into(),
    }
}
