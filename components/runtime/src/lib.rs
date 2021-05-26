use language::{
    abstract_syntax_tree::node::{Node, NumberLiteral},
    type_::{Arrow, Trait, Type},
};

pub trait Runtime {
    type Code;
    type Error;

    fn generate_code(ir: Node) -> Self::Code;
    fn run(code: Self::Code) -> Result<ComputedValue, Self::Error>;
}

/// A struct for a computed value with its type and encoding.
pub struct ComputedValue {
    pub type_: Type,
    pub encoded_value: EncodedValue,
}

pub enum EncodedValue {
    Unit,
    Label(String),
    Bool(bool),
    String(String),
    Number(NumberLiteral),
    Array(Vec<EncodedValue>),
    Product(Vec<EncodedValue>),
    Sum(Box<EncodedValue>),
    Type(Type),
    Function(Node),
    Effect {
        class: Trait,
        effect: Arrow,
    },
    Effectful {
        item: Box<EncodedValue>,
        class: Trait,
        handled: bool,
    },
}
