use crate::syntax::ast::Node;

use super::ir::IR;

pub fn analyze(node: Node) -> IR {
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
        Perform { effect, argument } => {
            todo!()
        }
        Handle {
            expression,
            effect,
            continuation,
            handler,
        } => {
            todo!()
        }
        Let {
            declarations,
            expression,
        } => {
            todo!()
        }
    }
}
