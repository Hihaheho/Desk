#[cfg_attr(feature = "withserde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Int(i64),
    Float(f64),
    Rational(i64, i64),
    Tuple(Vec<Value>),
    Variant { id: usize, value: Box<Value> },
}
