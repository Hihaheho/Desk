use components::{flat_node::Children, patch::ChildrenPatch};

use super::ChildrenPatchApplier;

impl ChildrenPatchApplier for &Children {
    fn apply_patch(self, patch: &ChildrenPatch) -> Children {
        let mut children = self.clone();
        match patch {
            ChildrenPatch::Insert { index, node } => {
                children.insert(*index, node.clone());
            }
            ChildrenPatch::Remove { index } => {
                children.remove(*index);
            }
            ChildrenPatch::Move { index, diff } => {
                let node = children.remove(*index);
                let new_index = *index as isize + *diff;
                children.insert(new_index as usize, node);
            }
            ChildrenPatch::Update { index, node } => {
                children.remove(*index);
                children.insert(*index, node.clone());
            }
        }
        children
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use deskc_ids::NodeId;

    #[test]
    fn insert() {
        let children = Vec::default();
        let node_id = NodeId::new();
        let children = children.apply_patch(&ChildrenPatch::Insert {
            index: 0,
            node: node_id.clone(),
        });
        assert_eq!(children, vec![node_id]);
    }

    #[test]
    fn remove() {
        let children = Vec::default();
        let node_a = NodeId::new();
        let node_b = NodeId::new();
        let children = children.apply_patch(&ChildrenPatch::Insert {
            index: 0,
            node: node_a.clone(),
        });
        let children = children.apply_patch(&ChildrenPatch::Insert {
            index: 1,
            node: node_b,
        });
        let children = children.apply_patch(&ChildrenPatch::Remove { index: 1 });
        assert_eq!(children, vec![node_a]);
    }

    #[test]
    fn move_() {
        let children = Vec::default();
        let node_a = NodeId::new();
        let node_b = NodeId::new();
        let children = children.apply_patch(&ChildrenPatch::Insert {
            index: 0,
            node: node_a.clone(),
        });
        let children = children.apply_patch(&ChildrenPatch::Insert {
            index: 1,
            node: node_b.clone(),
        });
        let children = children.apply_patch(&ChildrenPatch::Move { index: 1, diff: -1 });
        assert_eq!(children, vec![node_b, node_a]);
    }

    #[test]
    fn update() {
        let children = Vec::default();
        let node_a = NodeId::new();
        let node_b = NodeId::new();
        let children = children.apply_patch(&ChildrenPatch::Insert {
            index: 0,
            node: node_a,
        });
        let children = children.apply_patch(&ChildrenPatch::Update {
            index: 0,
            node: node_b.clone(),
        });
        assert_eq!(children, vec![node_b]);
    }
}
