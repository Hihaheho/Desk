#[derive(Clone, Debug, PartialEq)]
pub struct Integer(pub i32);

impl From<i32> for Integer {
    fn from(from: i32) -> Self {
        Self(from)
    }
}
