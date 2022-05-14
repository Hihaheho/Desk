use std::collections::HashMap;

use deskc_ids::{FileId, NodeId};
use hir::expr::Expr;
use types::Type;

use crate::{
    content::Content,
    patch::{AttributePatch, ChildrenPatch, ContentPatch},
};

pub type Children = Vec<NodeId>;
pub type Attributes = HashMap<Type, Expr>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlatNode {
    pub file_id: FileId,
    pub content: Content,
    pub children: Children,
    pub attributes: Attributes,
}

impl FlatNode {
    pub fn new(file_id: FileId, content: Content) -> Self {
        Self {
            file_id,
            content,
            children: Children::default(),
            attributes: Attributes::default(),
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

    pub fn patch_children(&mut self, patch: &ChildrenPatch) {
        match patch {
            ChildrenPatch::Insert { index, node } => {
                self.children.insert(*index, node.clone());
            }
            ChildrenPatch::Remove { index } => {
                self.children.remove(*index);
            }
            ChildrenPatch::Move { index, diff } => {
                let node = self.children.remove(*index);
                let new_index = *index as isize + *diff;
                self.children.insert(new_index as usize, node);
            }
            ChildrenPatch::Update { index, node } => {
                self.children.remove(*index);
                self.children.insert(*index, node.clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use hir::expr::Literal;

    use super::*;

    #[test]
    fn update() {
        let mut flat_node = FlatNode::new(FileId::new(), Content::String("a".into()));
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
        let mut flat_node = FlatNode::new(FileId::new(), Content::String("a".into()));
        flat_node.patch_attribute(&AttributePatch::Update {
            key: Type::Number,
            value: Box::new(Expr::Literal(Literal::Integer(1))),
        });
        flat_node.patch_attribute(&AttributePatch::Remove { key: Type::Number });

        assert_eq!(flat_node.attributes.get(&Type::Number), None,);
    }

    #[test]
    fn children_insert() {
        let mut flat_node = FlatNode::new(FileId::new(), Content::String("a".into()));
        let node_id = NodeId::new();
        flat_node.patch_children(&ChildrenPatch::Insert {
            index: 0,
            node: node_id.clone(),
        });
        assert_eq!(flat_node.children, vec![node_id]);
    }

    #[test]
    fn children_remove() {
        let mut flat_node = FlatNode::new(FileId::new(), Content::String("a".into()));
        let node_a = NodeId::new();
        let node_b = NodeId::new();
        flat_node.patch_children(&ChildrenPatch::Insert {
            index: 0,
            node: node_a.clone(),
        });
        flat_node.patch_children(&ChildrenPatch::Insert {
            index: 1,
            node: node_b,
        });
        flat_node.patch_children(&ChildrenPatch::Remove { index: 1 });
        assert_eq!(flat_node.children, vec![node_a]);
    }

    #[test]
    fn children_move_() {
        let mut flat_node = FlatNode::new(FileId::new(), Content::String("a".into()));
        let node_a = NodeId::new();
        let node_b = NodeId::new();
        flat_node.patch_children(&ChildrenPatch::Insert {
            index: 0,
            node: node_a.clone(),
        });
        flat_node.patch_children(&ChildrenPatch::Insert {
            index: 1,
            node: node_b.clone(),
        });
        flat_node.patch_children(&ChildrenPatch::Move { index: 1, diff: -1 });
        assert_eq!(flat_node.children, vec![node_b, node_a]);
    }

    #[test]
    fn children_update() {
        let mut flat_node = FlatNode::new(FileId::new(), Content::String("a".into()));
        let node_a = NodeId::new();
        let node_b = NodeId::new();
        flat_node.patch_children(&ChildrenPatch::Insert {
            index: 0,
            node: node_a,
        });
        flat_node.patch_children(&ChildrenPatch::Update {
            index: 0,
            node: node_b.clone(),
        });
        assert_eq!(flat_node.children, vec![node_b]);
    }
}
