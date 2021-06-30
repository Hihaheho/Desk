pub mod sugar;

use language::code::node::NumberLiteral;

#[derive(Clone, Debug)]
pub struct InputId(String);

impl<T: Into<String>> From<T> for InputId {
    fn from(from: T) -> Self {
        InputId(from.into())
    }
}

#[derive(Clone, Debug)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum Component {
    Blank,
    Label(String),
    InputString {
        id: InputId,
        value: String,
    },
    InputNumber {
        id: InputId,
        value: NumberLiteral,
    },
    Array {
        orientation: Orientation,
        items: Vec<Component>,
    },
}
