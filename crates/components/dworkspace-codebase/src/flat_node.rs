use std::collections::HashMap;

use deskc_ids::NodeId;
use dson::Dson;
use hir::expr::Expr;
use ty::Type;

use crate::{
    content::Content,
    patch::{AttributePatch, ContentPatch, OperandPatch},
    rules::{NodeOperation, Rules},
};

pub type Operands = Vec<NodeId>;
pub type Attributes = HashMap<Type, Dson>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlatNode {
    pub content: Content,
    pub operands: Operands,
    pub attributes: Attributes,
    pub rules: Rules<NodeOperation>,
    pub operand_rules: Rules<NodeOperation>,
}

impl FlatNode {
    pub fn new(content: Content) -> Self {
        Self {
            content,
            operands: Operands::default(),
            attributes: Attributes::default(),
            rules: Rules::default(),
            operand_rules: Rules::default(),
        }
    }

    pub fn patch_content(&mut self, patch: &ContentPatch) {
        match patch {
            ContentPatch::Replace(content) => self.content = content.clone(),
            _ => todo!(),
        }
    }

    pub fn patch_attribute(&mut self, patch: &AttributePatch) {
        match patch {
            AttributePatch::Update { key, value } => {
                self.attributes.insert(key.clone(), value.clone());
            }
            AttributePatch::Remove { key } => {
                self.attributes.remove(key);
            }
        }
    }

    pub fn patch_children(&mut self, patch: &OperandPatch) {
        match patch {
            OperandPatch::Insert {
                index,
                node_id: node,
            } => {
                self.operands.insert(*index, node.clone());
            }
            OperandPatch::Remove { index } => {
                self.operands.remove(*index);
            }
            OperandPatch::Move {
                from: index,
                to: next,
            } => {
                let node = self.operands.remove(*index);
                self.operands.insert(*next, node);
            }
        }
    }

    pub fn rules(mut self, rules: Rules<NodeOperation>) -> Self {
        self.rules = rules;
        self
    }

    pub fn operand_rules(mut self, rules: Rules<NodeOperation>) -> Self {
        self.operand_rules = rules;
        self
    }

    pub fn operands(mut self, operands: Operands) -> Self {
        self.operands = operands;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update() {
        let mut flat_node = FlatNode::new(Content::String("a".into()));
        flat_node.patch_attribute(&AttributePatch::Update {
            key: Type::Real,
            value: 1.into(),
        });
        assert_eq!(flat_node.attributes.get(&Type::Real), Some(&1.into()));
    }

    #[test]
    fn remove() {
        let mut flat_node = FlatNode::new(Content::String("a".into()));
        flat_node.patch_attribute(&AttributePatch::Update {
            key: Type::Real,
            value: 1.into(),
        });
        flat_node.patch_attribute(&AttributePatch::Remove { key: Type::Real });

        assert_eq!(flat_node.attributes.get(&Type::Real), None,);
    }

    #[test]
    fn operands_insert() {
        let mut flat_node = FlatNode::new(Content::String("a".into()));
        let node_id = NodeId::new();
        flat_node.patch_children(&OperandPatch::Insert {
            index: 0,
            node_id: node_id.clone(),
        });
        assert_eq!(flat_node.operands, vec![node_id]);
    }

    #[test]
    fn operands_remove() {
        let mut flat_node = FlatNode::new(Content::String("a".into()));
        let node_a = NodeId::new();
        let node_b = NodeId::new();
        flat_node.patch_children(&OperandPatch::Insert {
            index: 0,
            node_id: node_a.clone(),
        });
        flat_node.patch_children(&OperandPatch::Insert {
            index: 1,
            node_id: node_b,
        });
        flat_node.patch_children(&OperandPatch::Remove { index: 1 });
        assert_eq!(flat_node.operands, vec![node_a]);
    }

    #[test]
    fn operands_move() {
        let mut flat_node = FlatNode::new(Content::String("a".into()));
        let node_a = NodeId::new();
        let node_b = NodeId::new();
        let node_c = NodeId::new();
        flat_node.patch_children(&OperandPatch::Insert {
            index: 0,
            node_id: node_a.clone(),
        });
        flat_node.patch_children(&OperandPatch::Insert {
            index: 1,
            node_id: node_b.clone(),
        });
        flat_node.patch_children(&OperandPatch::Insert {
            index: 2,
            node_id: node_c.clone(),
        });
        flat_node.patch_children(&OperandPatch::Move { from: 1, to: 0 });
        flat_node.patch_children(&OperandPatch::Move { from: 1, to: 2 });
        assert_eq!(flat_node.operands, vec![node_b, node_c, node_a]);
    }
}
