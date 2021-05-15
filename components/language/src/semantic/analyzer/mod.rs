use crate::syntax::ast::Node;

use super::ir::Code;

pub fn analyze(node: Node) -> Code {
    use Node::*;
    match node {
        Literal { literal_value } => {
            todo!()
        }
        Construct { construct } => {
            todo!()
        }
        Variable(_) => {
            todo!()
        }
        Function {
            parameter,
            expression,
        } => {
            todo!()
        }
        Apply { function, argument } => {
            todo!()
        }
        Perform {
            effect,
            argument,
            emonad_id,
        } => {
            todo!()
        }
        Handle { handlers } => {
            todo!()
        }
        Let {
            declarations,
            expression,
        } => {
            todo!()
        }
        Match {
            expression,
            patterns,
        } => {
            todo!()
        }
    }
}
