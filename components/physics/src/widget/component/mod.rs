mod integer;
pub mod sugar;

pub use integer::*;

#[derive(Clone, Debug, PartialEq)]
pub struct InputId(String);

impl<T: Into<String>> From<T> for InputId {
    fn from(from: T) -> Self {
        InputId(from.into())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub enum Component {
    Blank,
    Label(String),
    InputString {
        id: InputId,
        value: String,
    },
    InputInteger {
        id: InputId,
        value: Integer,
    },
    Array {
        orientation: Orientation,
        items: Vec<Component>,
    },
}

impl Default for Component {
    fn default() -> Self {
        Component::Blank
    }
}
