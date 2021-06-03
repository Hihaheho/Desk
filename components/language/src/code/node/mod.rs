mod apply_binary_operator;
mod apply_function;
mod apply_unary_operator;
mod handle;
mod let_;
mod node;
mod perform;
pub mod sugar;

use crate::type_::Type;

#[derive(Clone, PartialEq, Debug)]
pub struct Identifier(u16);
#[derive(Clone, PartialEq, Debug)]
pub struct Function {}

#[derive(Clone, PartialEq, Debug)]
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

#[derive(Clone, PartialEq, Debug)]
pub enum NumberLiteral {
    /// integer, rational number, or float
    Integer(i32), // num-bigint?
    Rational(i32, i32),
    Float(f64), // bigdecimal?
}

#[derive(Clone, PartialEq, Debug)]
pub struct Node {
    pub data: NodeData,
    pub type_: Type,
    pub metadata: Option<u16>,
}

/// An enum for an AST Node without type annotation itself.
#[derive(Clone, PartialEq, Debug)]
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
        operand: Box<Node>,
    },
    ApplyBinaryOperator {
        operator: BinaryOperator,
        operands: (Box<Node>, Box<Node>),
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

#[derive(Clone, PartialEq, Debug)]
pub enum UnaryOperator {
    /// - Negation
    Neg,
    /// ! Logical negation
    Not,
    /// Absolute
    Abs,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BinaryOperator {
    Arithmetic(BinaryArithmeticOperator),
    Logical(BinaryLogicalOperator),
    ConcatString,
    IndexString,
}

#[derive(Copy, Clone, PartialEq, Debug)]
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

#[derive(Copy, Clone, PartialEq, Debug)]
/// Binary logical operators
pub enum BinaryLogicalOperator {
    And,
    Or,
}

#[derive(Clone, PartialEq, Debug)]
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

impl Node {
    pub fn reduce(&self) -> Node {
        node::reduce(self)
    }
}

#[cfg(test)]
mod test {
    use crate::code::node::sugar;

    #[test]
    fn reduce() {
        assert_eq!(sugar::integer(2).reduce(), sugar::integer(2))
    }
}
