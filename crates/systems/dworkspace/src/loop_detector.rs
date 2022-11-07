use std::sync::Arc;

use components::{event::Event, patch::OperandPatch, snapshot::Snapshot};
use deskc_ids::NodeId;
use parking_lot::Mutex;

use crate::descendants::{Descendants, DescendantsQueries};

#[derive(Default)]
pub struct LoopDetector {
    // salsa database is not Sync
    pub operand: Mutex<Descendants>,
}

impl LoopDetector {
    pub fn does_make_loop_insert_operand(&self, node_id: &NodeId, operand_id: &NodeId) -> bool {
        node_id == operand_id
            || self
                .operand
                .lock()
                .does_make_loop(node_id.clone(), operand_id.clone())
    }

    pub fn handle_event(&mut self, snapshot: &Snapshot, event: &Event) {
        match event {
            Event::PatchOperand {
                node_id,
                patch:
                    OperandPatch::Insert {
                        node_id: operand, ..
                    },
            } => {
                let mut lock = self.operand.lock();
                let mut operands = lock.node(node_id.clone()).as_ref().clone();
                operands.insert(operand.clone());
                lock.set_node(node_id.clone(), Arc::new(operands))
            }
            Event::PatchOperand {
                node_id,
                patch: OperandPatch::Remove { index },
            } => {
                let mut lock = self.operand.lock();
                let mut operands = lock.node(node_id.clone()).as_ref().clone();
                // asserted existence
                if let Some(operand) = snapshot
                    .flat_nodes
                    .get(node_id)
                    .and_then(|node| node.operands.get(*index))
                {
                    operands.remove(operand);
                    lock.set_node(node_id.clone(), Arc::new(operands))
                }
            }
            Event::CreateNode {
                node_id,
                content: _,
            } => {
                self.operand
                    .lock()
                    .set_node(node_id.clone(), Default::default());
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use components::{content::Content, flat_node::FlatNode};
    use deskc_ids::NodeId;

    use crate::descendants::DescendantsQueries;

    use super::*;

    #[test]
    fn detect_for_inserting_operand() {
        let node_a = NodeId::new();
        let node_b = NodeId::new();
        let detector = LoopDetector::default();
        detector.operand.lock().set_node(
            node_a.clone(),
            Arc::new([node_b.clone()].into_iter().collect()),
        );
        detector
            .operand
            .lock()
            .set_node(node_b.clone(), Arc::new([].into_iter().collect()));
        assert_eq!(
            detector.does_make_loop_insert_operand(&node_a, &node_b),
            false
        );
        assert_eq!(
            detector.does_make_loop_insert_operand(&node_b, &node_a),
            true
        );
    }

    #[test]
    fn detect_self_loop_by_operand() {
        let node_id = NodeId::new();
        let detector = LoopDetector::default();
        assert_eq!(
            detector.does_make_loop_insert_operand(&node_id, &node_id),
            true
        );
    }

    #[test]
    fn handle_create_node() {
        let node_id = NodeId::new();
        let mut detector = LoopDetector::default();
        detector.handle_event(
            &Snapshot::default(),
            &Event::CreateNode {
                node_id: node_id.clone(),
                content: Content::Integer(0),
            },
        );
        assert_eq!(
            detector.operand.lock().node(node_id.clone()),
            Default::default()
        );
    }

    #[test]
    fn handle_insert_operand() {
        let mut detector = LoopDetector::default();
        let node_a = NodeId::new();
        let node_b = NodeId::new();
        let node_c = NodeId::new();
        detector.operand.lock().set_node(
            node_a.clone(),
            Arc::new([node_c.clone()].into_iter().collect()),
        );
        detector
            .operand
            .lock()
            .set_node(node_b.clone(), Arc::new([].into_iter().collect()));
        detector
            .operand
            .lock()
            .set_node(node_c.clone(), Arc::new([].into_iter().collect()));
        detector.handle_event(
            &Snapshot::default(),
            &Event::PatchOperand {
                node_id: node_a.clone(),
                patch: OperandPatch::Insert {
                    node_id: node_b.clone(),
                    index: 0,
                },
            },
        );
        assert_eq!(
            detector.operand.lock().descendants(node_a),
            Arc::new([node_b, node_c].into_iter().collect())
        );
    }

    #[test]
    fn handle_remove_operand() {
        let mut detector = LoopDetector::default();
        let node_a = NodeId::new();
        let node_b = NodeId::new();
        let node_c = NodeId::new();
        detector.operand.lock().set_node(
            node_a.clone(),
            Arc::new([node_b.clone(), node_c.clone()].into_iter().collect()),
        );
        detector
            .operand
            .lock()
            .set_node(node_b.clone(), Arc::new([].into_iter().collect()));
        detector
            .operand
            .lock()
            .set_node(node_c.clone(), Arc::new([].into_iter().collect()));
        let mut snapshot = Snapshot::default();
        snapshot.flat_nodes.insert(
            node_a.clone(),
            FlatNode::new(Content::Integer(1)).operands(vec![node_b]),
        );
        detector.handle_event(
            &snapshot,
            &Event::PatchOperand {
                node_id: node_a.clone(),
                patch: OperandPatch::Remove { index: 0 },
            },
        );
        assert_eq!(
            detector.operand.lock().descendants(node_a),
            Arc::new([node_c].into_iter().collect())
        );
    }
}
