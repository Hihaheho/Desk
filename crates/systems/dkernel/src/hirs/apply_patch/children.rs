use dkernel_card::{
    flat_node::{Children, NodeRef},
    patch::ChildrenPatch,
};

use super::ChildrenPatchApplier;

impl ChildrenPatchApplier for Children {
    fn apply_patch(mut self, patch: &ChildrenPatch) -> Self {
        match patch {
            ChildrenPatch::Insert { index, node } => {
                // Reserve space for the new node.
                self.reserve_exact(*index);
                // Fill blank space with holes
                for _ in 0..(*index - self.len()) {
                    self.push(NodeRef::Hole);
                }
                self.insert(*index, node.clone());
            }
            ChildrenPatch::Remove { index } => {
                self.remove(*index);
            }
            ChildrenPatch::Move { index, diff } => {
                let node = self.remove(*index);
                let new_index = *index as isize + *diff;
                self.insert(new_index as usize, node);
            }
            ChildrenPatch::Update { index, node } => {
                self.remove(*index);
                self.insert(*index, node.clone());
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dkernel_card::node::NodeId;
    use uuid::Uuid;

    #[test]
    fn insert() {
        let children = Children::default();
        let node_id = NodeId(Uuid::new_v4());
        let children = children.apply_patch(&ChildrenPatch::Insert {
            index: 1,
            node: NodeRef::Node(node_id.clone()),
        });
        assert_eq!(children, vec![NodeRef::Hole, NodeRef::Node(node_id)]);
    }

    #[test]
    fn remove() {
        let children = Children::default();
        let node_id = NodeId(Uuid::new_v4());
        let children = children.apply_patch(&ChildrenPatch::Insert {
            index: 1,
            node: NodeRef::Node(node_id.clone()),
        });
        let children = children.apply_patch(&ChildrenPatch::Remove { index: 1 });
        assert_eq!(children, vec![NodeRef::Hole]);
    }

    #[test]
    fn move_() {
        let children = Children::default();
        let node_id = NodeId(Uuid::new_v4());
        let children = children.apply_patch(&ChildrenPatch::Insert {
            index: 1,
            node: NodeRef::Node(node_id.clone()),
        });
        let children = children.apply_patch(&ChildrenPatch::Move { index: 1, diff: -1 });
        assert_eq!(children, vec![NodeRef::Node(node_id), NodeRef::Hole]);
    }

    #[test]
    fn update() {
        let children = Children::default();
        let node_id = NodeId(Uuid::new_v4());
        let children = children.apply_patch(&ChildrenPatch::Insert {
            index: 1,
            node: NodeRef::Node(node_id.clone()),
        });
        let children = children.apply_patch(&ChildrenPatch::Update {
            index: 0,
            node: NodeRef::Node(node_id.clone()),
        });
        assert_eq!(
            children,
            vec![NodeRef::Node(node_id.clone()), NodeRef::Node(node_id)]
        );
    }
}
