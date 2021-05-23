use crate::type_::Type;

pub struct Identifier(u16);
pub struct Function {}

pub enum LiteralValue {
    Unit,
    Label(String),
    Bool(bool),
    String(String),
    Number(NumberLiteral),
    Array(Vec<Node>),
    Product(Vec<Node>),
    Sum(Box<Node>),
    Type(Type),
}

pub enum NumberLiteral {
    /// integer, rational number, or float
    Integer(i64), // num-bigint?
    Rational(i64, i64),
    Float(f64), // bigdecimal?
}

pub struct Node {
    data: NodeData,
    type_annotation: Option<Type>,
    metadata: NodeMetadata,
}

#[derive(Default)]
pub struct NodeMetadata {
    pub inferred_type: Option<Type>,
}

/// An enum for an AST Node without type annotation itself.
pub enum NodeData {
    Literal {
        value: LiteralValue,
    },
    Let {
        variable: Identifier,
        value: Box<Node>,
        expression: Box<Node>,
    },
    Variable {
        identifier: Identifier,
    },
    ApplyUnaryOperator {
        operator: UnaryOperator,
        operands: (Box<Node>, Box<Node>),
    },
    ApplyBinaryOperator {
        operator: BinaryOperator,
        operand: Box<Node>,
    },
    Function {
        parameter: Identifier,
        expression: Box<Node>,
    },
    ApplyFunction {
        function: Box<Node>,
        argument: Box<Node>,
    },
    Perform {
        effect: Box<Node>,
        argument: Box<Node>,
    },
    Handle {
        expression: Box<Node>,
        acc: Box<Node>,
        handlers: Vec<Node>,
    },
}

pub enum UnaryOperator {
    /// - Negation
    Neg,
    /// ! Logical negation
    Not,
    /// Absolute
    Abs,
}

pub enum BinaryOperator {
    Arithmetic(BinaryArithmeticOperator),
    Logical(BinaryLogicalOperator),
    ConcatString,
    IndexString,
}

/// Binary arithmetic operators
pub enum BinaryArithmeticOperator {
    /// + Addition
    Add,
    /// - Subtraction
    Sub,
    /// * Multiplication
    Mul,
    /// / Dividion
    Div,
    /// mod
    Mod,
}

/// Binary logical operators
pub enum BinaryLogicalOperator {
    And,
    Or,
}

/// Comparison operators
pub enum ComparisonOperator {
    /// == Equal to
    Eq,
    /// != Not equal to
    Ne,
    /// > Greater than
    Gt,
    /// < Less than
    Lt,
    /// >= Greator than or equal to
    Ge,
    /// <= Less than or equal to
    Le,
}
