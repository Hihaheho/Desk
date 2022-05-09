use std::collections::{HashMap, HashSet};

use deskc_ids::CardId;
use dkernel_card::{flat_node::NodeRef, node::NodeId, patch::ChildrenPatch};

use crate::{event::Event, snapshot::Snapshot};

#[derive(Default)]
/// Sets of node IDs that refers a node.
pub struct References(pub HashMap<NodeId, HashSet<NodeId>>);

impl References {
    pub fn handle_event(&mut self, snapshot: &Snapshot, event: &Event) {
        match event {
            Event::PatchChildren { node_id, patch } => match patch {
                ChildrenPatch::Insert { index, node } => {
                    if let NodeRef::Node(referred) = node {
                        self.0
                            .entry(referred.clone())
                            .or_default()
                            .insert(node_id.clone());
                    }
                }

                ChildrenPatch::Remove(_) => todo!(),
                ChildrenPatch::Move { index, diff } => todo!(),
                ChildrenPatch::Update { index, node } => todo!(),
            },
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use dkernel_card::{flat_node::NodeRef, patch::ChildrenPatch};
    use uuid::Uuid;

    use super::*;

    #[test]
    fn insert_node() {
        let mut cards = References::default();
        let node_a = NodeId(Uuid::new_v4());
        let node_b = NodeId(Uuid::new_v4());
        cards.handle_event(
            &Default::default(),
            &Event::PatchChildren {
                node_id: node_a.clone(),
                patch: ChildrenPatch::Insert {
                    index: 0,
                    node: NodeRef::Node(node_b.clone()),
                },
            },
        );
        assert_eq!(
            cards.0,
            [(node_b, [node_a].into_iter().collect())]
                .into_iter()
                .collect()
        )
    }
}
