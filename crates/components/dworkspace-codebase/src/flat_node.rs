use std::collections::HashMap;

use deskc_ids::NodeId;
use dson::Dson;
use ty::Type;

use crate::{
    content::Content,
    patch::{AttributePatch, ContentPatch, OperandPatch, OperandPosition},
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
        fn find_index(operands: &Operands, node_id: &NodeId) -> usize {
            operands.iter().position(|&id| id == *node_id).unwrap()
        }
        fn get_index(operands: &Operands, position: &OperandPosition) -> usize {
            match position {
                OperandPosition::At(index) => *index,
                OperandPosition::First => 0,
                OperandPosition::Last => operands.len(),
                OperandPosition::Before(node_id) => find_index(operands, node_id),
                OperandPosition::After(node_id) => find_index(operands, node_id) + 1,
            }
        }
        match patch {
            OperandPatch::Insert { node_id, position } => {
                let index = get_index(&self.operands, position);
                self.operands.insert(index, *node_id);
            }
            OperandPatch::Remove { node_id } => {
                let index = self.operands.iter().position(|&id| id == *node_id).unwrap();
                self.operands.remove(index);
            }
            OperandPatch::Move { node_id, position } => {
                let origin_index = find_index(&self.operands, node_id);
                let node = self.operands.remove(origin_index);
                let target = get_index(&self.operands, position);
                self.operands.insert(target, node);
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
        let a = NodeId::new();
        flat_node.patch_children(&OperandPatch::Insert {
            node_id: a.clone(),
            position: OperandPosition::First,
        });
        assert_eq!(flat_node.operands, vec![a]);
    }

    #[test]
    fn operands_remove() {
        let mut flat_node = FlatNode::new(Content::String("a".into()));
        let node_a = NodeId::new();
        let node_b = NodeId::new();
        flat_node.patch_children(&OperandPatch::Insert {
            position: OperandPosition::First,
            node_id: node_a.clone(),
        });
        flat_node.patch_children(&OperandPatch::Insert {
            position: OperandPosition::Last,
            node_id: node_b,
        });
        flat_node.patch_children(&OperandPatch::Remove { node_id: node_b });
        assert_eq!(flat_node.operands, vec![node_a]);
    }

    #[test]
    fn operands_move() {
        let mut flat_node = FlatNode::new(Content::String("a".into()));
        let node_a = NodeId::new();
        let node_b = NodeId::new();
        let node_c = NodeId::new();
        flat_node.patch_children(&OperandPatch::Insert {
            position: OperandPosition::At(0),
            node_id: node_b.clone(),
        });
        flat_node.patch_children(&OperandPatch::Insert {
            position: OperandPosition::Before(node_b),
            node_id: node_a.clone(),
        });
        flat_node.patch_children(&OperandPatch::Insert {
            position: OperandPosition::After(node_b),
            node_id: node_c.clone(),
        });
        flat_node.patch_children(&OperandPatch::Move {
            node_id: node_b,
            position: OperandPosition::First,
        });
        flat_node.patch_children(&OperandPatch::Move {
            node_id: node_a,
            position: OperandPosition::Last,
        });
        assert_eq!(flat_node.operands, vec![node_b, node_c, node_a]);
    }
}
