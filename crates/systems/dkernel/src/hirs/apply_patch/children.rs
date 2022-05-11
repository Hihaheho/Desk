use components::patch::ChildrenPatch;
use deskc_ids::NodeId;

use super::ChildrenPatchApplier;

impl ChildrenPatchApplier for Vec<NodeId> {
    fn apply_patch(mut self, patch: &ChildrenPatch) -> Self {
        match patch {
            ChildrenPatch::Insert { index, node } => {
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
    use deskc_ids::NodeId;
    use uuid::Uuid;

    #[test]
    fn insert() {
        let children = Vec::default();
        let node_id = NodeId(Uuid::new_v4());
        let children = children.apply_patch(&ChildrenPatch::Insert {
            index: 0,
            node: node_id.clone(),
        });
        assert_eq!(children, vec![node_id]);
    }

    #[test]
    fn remove() {
        let children = Vec::default();
        let node_a = NodeId(Uuid::new_v4());
        let node_b = NodeId(Uuid::new_v4());
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
        let node_a = NodeId(Uuid::new_v4());
        let node_b = NodeId(Uuid::new_v4());
        let children = children.apply_patch(&ChildrenPatch::Insert {
            index: 0,
            node: node_a.clone(),
        });
        let children = children.apply_patch(&ChildrenPatch::Insert {
            index: 1,
            node: node_b.clone(),
        });
        let children = children.apply_patch(&ChildrenPatch::Remove { index: 1 });
        let children = children.apply_patch(&ChildrenPatch::Move { index: 1, diff: -1 });
        assert_eq!(children, vec![node_b, node_a]);
    }

    #[test]
    fn update() {
        let children = Vec::default();
        let node_a = NodeId(Uuid::new_v4());
        let node_b = NodeId(Uuid::new_v4());
        let children = children.apply_patch(&ChildrenPatch::Insert {
            index: 0,
            node: node_a.clone(),
        });
        let children = children.apply_patch(&ChildrenPatch::Insert {
            index: 1,
            node: node_b,
        });
        assert_eq!(children, vec![node_b]);
    }
}
