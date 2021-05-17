use protocol::id::Id;

use crate::typing::type_::Type;

pub struct Identifier(u16);
pub struct Function {}

pub struct AnnotatableNode<T = Node> {
    item: T,
    type_annotation: Option<Type>,
}

pub enum LiteralValue {
    String(String),
    I32(i32),
    F32(f32),
}

/// An enum for an AST Node without type annotation itself.
pub enum Node {
    Literal {
        literal_value: LiteralValue,
    },
    Construct {
        construct: Construct,
    },
    Let {
        declarations: Vec<AnnotatableNode<Construct>>,
        expression: Box<AnnotatableNode>,
    },
    Variable(Identifier),
    Function {
        parameter: AnnotatableNode<Construct>,
        expression: Box<AnnotatableNode>,
    },
    Apply {
        function: Box<AnnotatableNode>,
        argument: Box<AnnotatableNode>,
    },
    Perform {
        effect: AnnotatableNode<Construct>,
        argument: Box<AnnotatableNode>,
    },
    Handle {
        expression: Box<AnnotatableNode>,
        effect: AnnotatableNode<Construct>,
        continuation: Identifier,
        handler: Box<AnnotatableNode>,
    },
}

pub struct ConstructorId(Id);

pub struct Construct {
    pub constructor_id: ConstructorId,
    pub arguments: Vec<AnnotatableNode>,
}
