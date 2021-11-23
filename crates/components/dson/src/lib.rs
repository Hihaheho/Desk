#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    String(String),
    Int(i64),
    Rational(i64, i64),
    Float(f64),
    Hole,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Dson {
    Literal(Literal),
    Product(Vec<Self>),
    Array(Vec<Self>),
    Set(Vec<Self>),
    Attr { attr: Box<Self>, expr: Box<Self> },
    Labeled { label: String, expr: Box<Self> },
}
