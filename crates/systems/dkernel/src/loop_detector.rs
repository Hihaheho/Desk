use std::{collections::HashSet, sync::Arc};

use components::{event::Event, patch::OperandsPatch, snapshot::Snapshot};
use parking_lot::Mutex;

use crate::descendants::{Descendants, DescendantsQueries};

#[derive(Default)]
pub struct LoopDetector {
    // salsa database is not Sync
    pub parent: Mutex<Descendants>,
    pub operand: Mutex<Descendants>,
}

impl LoopDetector {
    pub fn does_make_loop(&self, event: &Event) -> bool {
        match event {
            Event::PatchOperands {
                node_id,
                patch: OperandsPatch::Insert { node: operand, .. },
            } => {
                node_id == operand
                    || self
                        .operand
                        .lock()
                        .does_make_loop(node_id.clone(), operand.clone())
            }
            Event::UpdateParent {
                node_id,
                parent: Some(parent),
            } => {
                node_id == parent
                    || self
                        .parent
                        .lock()
                        .does_make_loop(node_id.clone(), parent.clone())
            }
            _ => false,
        }
    }

    pub fn handle_event(&mut self, snapshot: &Snapshot, event: &Event) {
        match event {
            Event::UpdateParent {
                node_id,
                parent: Some(parent),
            } => self.parent.lock().set_node(
                node_id.clone(),
                Arc::new([parent.clone()].into_iter().collect()),
            ),
            Event::UpdateParent {
                node_id,
                parent: None,
            } => self
                .parent
                .lock()
                .set_node(node_id.clone(), Default::default()),
            Event::PatchOperands {
                node_id,
                patch: OperandsPatch::Insert { node: operand, .. },
            } => {
                let mut lock = self.operand.lock();
                let mut operands = lock.node(node_id.clone()).as_ref().clone();
                operands.insert(operand.clone());
                lock.set_node(node_id.clone(), Arc::new(operands))
            }
            Event::PatchOperands {
                node_id,
                patch: OperandsPatch::Remove { index },
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
            Event::AddNode {
                parent, node_id, ..
            } => {
                if let Some(parent) = parent {
                    self.parent.lock().set_node(
                        node_id.clone(),
                        Arc::new([parent.clone()].into_iter().collect()),
                    );
                }
                self.operand
                    .lock()
                    .set_node(node_id.clone(), Arc::new(Default::default()));
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
            detector.does_make_loop(&Event::PatchOperands {
                node_id: node_a.clone(),
                patch: OperandsPatch::Insert {
                    node: node_b.clone(),
                    index: 0,
                },
            }),
            false
        );
        assert_eq!(
            detector.does_make_loop(&Event::PatchOperands {
                node_id: node_b.clone(),
                patch: OperandsPatch::Insert {
                    node: node_a.clone(),
                    index: 0,
                },
            }),
            true
        );
    }

    #[test]
    fn detect_for_set_parent() {
        let node_a = NodeId::new();
        let node_b = NodeId::new();
        let detector = LoopDetector::default();
        detector.parent.lock().set_node(
            node_a.clone(),
            Arc::new([node_b.clone()].into_iter().collect()),
        );
        detector
            .parent
            .lock()
            .set_node(node_b.clone(), Arc::new([].into_iter().collect()));
        assert_eq!(
            detector.does_make_loop(&Event::UpdateParent {
                node_id: node_a.clone(),
                parent: Some(node_b.clone()),
            }),
            false
        );
        assert_eq!(
            detector.does_make_loop(&Event::UpdateParent {
                node_id: node_b.clone(),
                parent: Some(node_a.clone()),
            }),
            true
        );
    }

    #[test]
    fn detect_self_loop_by_operand() {
        let node_id = NodeId::new();
        let detector = LoopDetector::default();
        assert_eq!(
            detector.does_make_loop(&Event::PatchOperands {
                node_id: node_id.clone(),
                patch: OperandsPatch::Insert {
                    node: node_id.clone(),
                    index: 0,
                },
            }),
            true
        );
    }

    #[test]
    fn detect_self_loop_by_parent() {
        let node_id = NodeId::new();
        let detector = LoopDetector::default();
        assert_eq!(
            detector.does_make_loop(&Event::UpdateParent {
                node_id: node_id.clone(),
                parent: Some(node_id.clone()),
            }),
            true
        );
    }

    #[test]
    fn handle_set_some_parent() {
        let mut detector = LoopDetector::default();
        let node_a = NodeId::new();
        let node_b = NodeId::new();
        detector
            .parent
            .lock()
            .set_node(node_a.clone(), Arc::new([].into_iter().collect()));
        detector
            .parent
            .lock()
            .set_node(node_b.clone(), Arc::new([].into_iter().collect()));
        detector.handle_event(
            &Snapshot::default(),
            &Event::UpdateParent {
                node_id: node_a.clone(),
                parent: Some(node_b.clone()),
            },
        );
        assert_eq!(
            detector.parent.lock().descendants(node_a),
            Arc::new([node_b].into_iter().collect())
        );
    }

    #[test]
    fn handle_set_none_parent() {
        let mut detector = LoopDetector::default();
        let node_a = NodeId::new();
        let node_b = NodeId::new();
        detector.parent.lock().set_node(
            node_a.clone(),
            Arc::new([node_b.clone()].into_iter().collect()),
        );
        detector
            .parent
            .lock()
            .set_node(node_b.clone(), Arc::new([].into_iter().collect()));
        detector.handle_event(
            &Snapshot::default(),
            &Event::UpdateParent {
                node_id: node_a.clone(),
                parent: None,
            },
        );
        assert_eq!(
            detector.parent.lock().descendants(node_a),
            Arc::new(Default::default())
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
            &Event::PatchOperands {
                node_id: node_a.clone(),
                patch: OperandsPatch::Insert {
                    node: node_b.clone(),
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
            &Event::PatchOperands {
                node_id: node_a.clone(),
                patch: OperandsPatch::Remove { index: 0 },
            },
        );
        assert_eq!(
            detector.operand.lock().descendants(node_a),
            Arc::new([node_c].into_iter().collect())
        );
    }

    #[test]
    fn handle_add_node_with_out_parent() {
        let mut detector = LoopDetector::default();
        let node_id = NodeId::new();
        detector.handle_event(
            &Snapshot::default(),
            &Event::AddNode {
                node_id: node_id.clone(),
                parent: None,
                content: Content::Integer(1),
            },
        );
        assert_eq!(
            detector.operand.lock().descendants(node_id),
            Arc::new([].into_iter().collect())
        );
    }

    #[test]
    fn handle_add_node_with_parent() {
        let mut detector = LoopDetector::default();
        let node_a = NodeId::new();
        let node_b = NodeId::new();
        detector
            .parent
            .lock()
            .set_node(node_a.clone(), Arc::new([].into_iter().collect()));
        detector.handle_event(
            &Snapshot::default(),
            &Event::AddNode {
                node_id: node_b.clone(),
                parent: Some(node_a.clone()),
                content: Content::Integer(1),
            },
        );
        assert_eq!(
            detector.operand.lock().descendants(node_b.clone()),
            Arc::new([].into_iter().collect())
        );
        assert_eq!(
            detector.parent.lock().descendants(node_b),
            Arc::new([node_a].into_iter().collect())
        );
    }
}
