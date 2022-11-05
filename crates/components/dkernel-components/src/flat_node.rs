use std::collections::HashMap;

use deskc_ids::NodeId;
use hir::expr::Expr;
use types::Type;

use crate::{
    content::Content,
    patch::{AttributePatch, ContentPatch, OperandsPatch},
    rules::{NodeOperation, Rules},
};

pub type Operands = Vec<NodeId>;
pub type Attributes = HashMap<Type, Expr>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlatNode {
    pub content: Content,
    pub operands: Operands,
    pub attributes: Attributes,
    pub rules: Rules<NodeOperation>,
    pub parent: Option<NodeId>,
}

impl FlatNode {
    pub fn new(content: Content) -> Self {
        Self {
            content,
            operands: Operands::default(),
            attributes: Attributes::default(),
            rules: Rules::default(),
            parent: None,
        }
    }

    pub fn patch_content(&mut self, patch: &ContentPatch) {
        match patch {
            ContentPatch::Replace(content) => self.content = content.clone(),
            ContentPatch::PatchString(_) => todo!(),
            ContentPatch::AddInteger(_) => todo!(),
            ContentPatch::AddFloat(_) => todo!(),
        }
    }

    pub fn patch_attribute(&mut self, patch: &AttributePatch) {
        match patch {
            AttributePatch::Update { key, value } => {
                self.attributes.insert(key.clone(), *value.clone());
            }
            AttributePatch::Remove { key } => {
                self.attributes.remove(key);
            }
        }
    }

    pub fn patch_children(&mut self, patch: &OperandsPatch) {
        match patch {
            OperandsPatch::Insert { index, node } => {
                self.operands.insert(*index, node.clone());
            }
            OperandsPatch::Remove { index } => {
                self.operands.remove(*index);
            }
            OperandsPatch::Move { index, diff } => {
                let node = self.operands.remove(*index);
                let new_index = *index as isize + *diff;
                self.operands.insert(new_index as usize, node);
            }
        }
    }

    pub fn rules(mut self, rules: Rules<NodeOperation>) -> Self {
        self.rules = rules;
        self
    }

    pub fn parent(mut self, parent: Option<NodeId>) -> Self {
        self.parent = parent;
        self
    }

    pub fn operands(mut self, operands: Operands) -> Self {
        self.operands = operands;
        self
    }
}

#[cfg(test)]
mod tests {
    use hir::expr::Literal;

    use super::*;

    #[test]
    fn update() {
        let mut flat_node = FlatNode::new(Content::String("a".into()));
        flat_node.patch_attribute(&AttributePatch::Update {
            key: Type::Number,
            value: Box::new(Expr::Literal(Literal::Integer(1))),
        });
        assert_eq!(
            flat_node.attributes.get(&Type::Number),
            Some(&Expr::Literal(Literal::Integer(1)))
        );
    }

    #[test]
    fn remove() {
        let mut flat_node = FlatNode::new(Content::String("a".into()));
        flat_node.patch_attribute(&AttributePatch::Update {
            key: Type::Number,
            value: Box::new(Expr::Literal(Literal::Integer(1))),
        });
        flat_node.patch_attribute(&AttributePatch::Remove { key: Type::Number });

        assert_eq!(flat_node.attributes.get(&Type::Number), None,);
    }

    #[test]
    fn operands_insert() {
        let mut flat_node = FlatNode::new(Content::String("a".into()));
        let node_id = NodeId::new();
        flat_node.patch_children(&OperandsPatch::Insert {
            index: 0,
            node: node_id.clone(),
        });
        assert_eq!(flat_node.operands, vec![node_id]);
    }

    #[test]
    fn operands_remove() {
        let mut flat_node = FlatNode::new(Content::String("a".into()));
        let node_a = NodeId::new();
        let node_b = NodeId::new();
        flat_node.patch_children(&OperandsPatch::Insert {
            index: 0,
            node: node_a.clone(),
        });
        flat_node.patch_children(&OperandsPatch::Insert {
            index: 1,
            node: node_b,
        });
        flat_node.patch_children(&OperandsPatch::Remove { index: 1 });
        assert_eq!(flat_node.operands, vec![node_a]);
    }

    #[test]
    fn operands_move_() {
        let mut flat_node = FlatNode::new(Content::String("a".into()));
        let node_a = NodeId::new();
        let node_b = NodeId::new();
        flat_node.patch_children(&OperandsPatch::Insert {
            index: 0,
            node: node_a.clone(),
        });
        flat_node.patch_children(&OperandsPatch::Insert {
            index: 1,
            node: node_b.clone(),
        });
        flat_node.patch_children(&OperandsPatch::Move { index: 1, diff: -1 });
        assert_eq!(flat_node.operands, vec![node_b, node_a]);
    }
}
