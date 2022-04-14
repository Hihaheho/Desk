use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Token {
    Comment(String),
    Ident(String),
    Int(i64),
    Str(String),
    // TODO: Float(i64, i64),
    Uuid(Uuid),
    Divide,
    Let,
    In,
    Perform,
    This,
    FromHere,
    TypeAnnotation,
    Trait,
    Attribute,
    Sum,
    Product,
    Comma,
    Dot,
    CommentBegin,
    CommentEnd,
    Substitution,
    Apply,
    ArrayBegin,
    ArrayEnd,
    SetBegin,
    SetEnd,
    Hole,
    Infer,
    Handle,
    Lambda,
    Arrow,
    EArrow,
    Include,
    Import,
    Export,
    Brands,
    Type,
    NumberType,
    StringType,
    Brand(String),
    Alias,
    A,
}
