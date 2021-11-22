use types::Type;

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    String(String),
    Int(i64),
    Rational(i64, i64),
    Float(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Product(Vec<Self>),
    Typed { ty: Type, expr: Box<Self> },
    Hole,
    Array(Vec<Self>),
    Set(Vec<Self>),
    Attr { attr: Box<Self>, expr: Box<Self> },
}
