use protocol::id::Id;

use crate::typing::r#type::{EMonadId, EffectId, Type};
use std::collections::HashMap;

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
        declarations: Vec<AnnotatableNode<Declaration>>,
        expression: Box<AnnotatableNode>,
    },
    Match {
        expression: Box<AnnotatableNode>,
        patterns: Vec<MatchBranch>,
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
        emonad_id: EMonadId,
        effect: EffectId,
        argument: Box<AnnotatableNode>,
    },
    Handle {
        handlers: HashMap<EffectId, Handler>,
    },
}

pub struct ConstructorId(Id);

pub struct Construct {
    pub constructor_id: ConstructorId,
    pub arguments: Vec<AnnotatableNode>,
}

pub struct MatchBranch {
    pattern: Construct,
    guards: Vec<Guard>,
    // not annotatable because all patterns in a match expression must be the same type.
    pub expression: Node,
}

pub struct Guard {
    // todo
}

pub struct Declaration {
    // Pattern is allowed
    pub variable: Construct,
    pub expression: AnnotatableNode,
}

pub struct Handler {
    pub effect: EffectId,
    pub continuation: Identifier,
    // not annotatable because all handlers in a handle expression must be the same type.
    pub expression: Node,
}
