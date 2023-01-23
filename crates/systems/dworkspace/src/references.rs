use std::{collections::HashSet, ops::DerefMut, sync::Arc};

use components::{
    event::Event,
    patch::OperandPatch,
    projection::Projection,
    rules::{NodeOperation, Rules},
};
use deskc_ids::NodeId;

#[salsa::query_group(KernelStorage)]
pub trait ReferencesQueries {
    #[salsa::input]
    fn node(&self, id: NodeId) -> Arc<HashSet<NodeId>>;
    #[salsa::input]
    fn operand_rules(&self, id: NodeId) -> Arc<Rules<NodeOperation>>;
    fn references(&self, id: NodeId) -> Arc<HashSet<NodeId>>;
    fn parent_rules(&self, id: NodeId) -> Option<Arc<Rules<NodeOperation>>>;
}

#[salsa::database(KernelStorage)]
#[derive(Default)]
pub struct References {
    storage: salsa::Storage<Self>,
    top_level_nodes: HashSet<NodeId>,
}

impl salsa::Database for References {}

fn references(db: &dyn ReferencesQueries, node_id: NodeId) -> Arc<HashSet<NodeId>> {
    let mut ret = db.node(node_id).as_ref().clone();
    let mut node_ids: Vec<NodeId> = ret.iter().cloned().collect();
    let mut next_node_ids;
    while !node_ids.is_empty() {
        next_node_ids = Vec::new();
        for node_id in node_ids {
            let children = db.references(node_id.clone());
            next_node_ids.extend(children.difference(&ret).cloned());
            ret = ret.union(&children).cloned().collect();
        }
        node_ids = next_node_ids.to_vec();
    }
    Arc::new(ret)
}

fn parent_rules(db: &dyn ReferencesQueries, node_id: NodeId) -> Option<Arc<Rules<NodeOperation>>> {
    db.references(node_id)
        .iter()
        .map(|node_id| db.operand_rules(node_id.clone()))
        .reduce(|rule1, rule2| Arc::new(rule1.intersection(&rule2)))
}

impl References {
    pub fn handle_event(&mut self, snapshot: &Projection, event: &Event) {
        match event {
            Event::CreateNode {
                node_id,
                content: _,
            } => {
                self.set_node(node_id.clone(), Arc::new(HashSet::new()));
                self.set_operand_rules(node_id.clone(), Arc::new(Rules::default()));
                self.top_level_nodes.insert(node_id.clone());
            }
            Event::PatchOperand {
                node_id,
                patch:
                    OperandPatch::Insert {
                        position: _,
                        node_id: operand_id,
                    },
            } => {
                let mut references = self.node(operand_id.clone()).as_ref().clone();
                references.insert(node_id.clone());
                self.set_node(operand_id.clone(), Arc::new(references));
                self.top_level_nodes.remove(operand_id);
            }
            Event::PatchOperand {
                node_id,
                patch: OperandPatch::Remove { node_id: removed },
            } => {
                let mut references = self.node(*removed).as_ref().clone();
                references.remove(node_id);
                if references.is_empty() {
                    self.top_level_nodes.insert(*removed);
                }
                self.set_node(*removed, Arc::new(references));
            }
            Event::UpdateOperandRules { node_id, rules } => {
                self.set_operand_rules(node_id.clone(), Arc::new(rules.clone()));
            }
            _ => {}
        }
    }
}

impl References {
    pub fn top_level_nodes(&self) -> impl Iterator<Item = &NodeId> {
        self.top_level_nodes.iter()
    }
}

#[cfg(test)]
mod tests {
    use components::{
        content::Content,
        flat_node::FlatNode,
        patch::{OperandPatch, OperandPosition},
    };

    use super::*;

    #[test]
    fn return_references() {
        // a - b - c + d
        //           - e
        let node_a = NodeId::new();
        let node_b = NodeId::new();
        let node_c = NodeId::new();
        let node_d = NodeId::new();
        let node_e = NodeId::new();
        let mut db = References::default();
        db.set_node(node_a, Arc::new([node_b.clone()].into_iter().collect()));
        db.set_node(
            node_b.clone(),
            Arc::new([node_c.clone()].into_iter().collect()),
        );
        db.set_node(
            node_c.clone(),
            Arc::new([node_d.clone(), node_e.clone()].into_iter().collect()),
        );
        db.set_node(node_d.clone(), Arc::new([].into_iter().collect()));
        db.set_node(node_e.clone(), Arc::new([].into_iter().collect()));
        assert_eq!(
            db.references(node_b),
            Arc::new([node_c, node_d, node_e].into_iter().collect())
        );
    }

    #[test]
    fn return_parent_rules() {
        // a - b - c + d
        //           - e
        let node_a = NodeId::new();
        let node_b = NodeId::new();
        let node_c = NodeId::new();
        let node_d = NodeId::new();
        let node_e = NodeId::new();
        let mut db = References::default();
        db.set_node(
            node_a.clone(),
            Arc::new([node_b.clone()].into_iter().collect()),
        );
        db.set_node(
            node_b.clone(),
            Arc::new([node_c.clone()].into_iter().collect()),
        );
        db.set_node(
            node_c.clone(),
            Arc::new([node_d.clone(), node_e.clone()].into_iter().collect()),
        );
        db.set_node(node_d.clone(), Arc::new([].into_iter().collect()));
        db.set_node(node_e.clone(), Arc::new([].into_iter().collect()));
        use NodeOperation::*;
        db.set_operand_rules(
            node_a,
            Arc::new(Rules {
                default: [RemoveNode].into_iter().collect(),
                ..Default::default()
            }),
        );
        db.set_operand_rules(
            node_b.clone(),
            Arc::new(Rules {
                default: [PatchString].into_iter().collect(),
                ..Default::default()
            }),
        );
        db.set_operand_rules(
            node_c,
            Arc::new(Rules {
                default: [UpdateInteger, UpdateReal].into_iter().collect(),
                ..Default::default()
            }),
        );
        db.set_operand_rules(
            node_d.clone(),
            Arc::new(Rules {
                default: [UpdateInteger, UpdateRational].into_iter().collect(),
                ..Default::default()
            }),
        );
        db.set_operand_rules(
            node_e,
            Arc::new(Rules {
                default: [UpdateInteger, UpdateReal].into_iter().collect(),
                ..Default::default()
            }),
        );
        assert_eq!(
            db.parent_rules(node_b),
            Some(Arc::new(Rules {
                default: [UpdateInteger].into_iter().collect(),
                ..Default::default()
            }))
        );
        assert_eq!(db.parent_rules(node_d), None);
    }

    #[test]
    fn handle_event_add_operand() {
        let mut db = References::default();
        let node_id = NodeId::new();
        let operand_id = NodeId::new();
        db.set_node(operand_id.clone(), Arc::new([].into_iter().collect()));
        db.handle_event(
            &Projection::default(),
            &Event::PatchOperand {
                node_id: node_id.clone(),
                patch: OperandPatch::Insert {
                    position: OperandPosition::First,
                    node_id: operand_id.clone(),
                },
            },
        );
        assert_eq!(
            db.node(operand_id),
            Arc::new([node_id].into_iter().collect())
        );
    }

    #[test]
    fn handle_event_create_node() {
        let mut db = References::default();
        let node_id = NodeId::new();
        db.handle_event(
            &Projection::default(),
            &Event::CreateNode {
                node_id: node_id.clone(),
                content: Content::Integer(1),
            },
        );
        assert_eq!(db.node(node_id.clone()), Arc::new([].into_iter().collect()));
        assert_eq!(db.operand_rules(node_id), Arc::new(Rules::default()));
    }

    #[test]
    fn handle_event_remove_operand() {
        let mut db = References::default();
        let mut snapshot = Projection::default();
        let node_id = NodeId::new();
        let operand_id = NodeId::new();
        snapshot.flat_nodes.insert(
            node_id.clone(),
            FlatNode::new(Content::Integer(1)).operands(vec![operand_id.clone()]),
        );
        db.set_node(
            operand_id.clone(),
            Arc::new([node_id.clone()].into_iter().collect()),
        );
        db.handle_event(
            &snapshot,
            &Event::PatchOperand {
                node_id,
                patch: OperandPatch::Remove {
                    node_id: operand_id,
                },
            },
        );
        assert_eq!(db.node(operand_id), Arc::new([].into_iter().collect()));
    }

    #[test]
    fn handle_event_update_operand_rules() {
        let mut db = References::default();
        let node_id = NodeId::new();
        db.handle_event(
            &Projection::default(),
            &Event::UpdateOperandRules {
                node_id: node_id.clone(),
                rules: Rules {
                    default: [NodeOperation::UpdateInteger].into_iter().collect(),
                    ..Default::default()
                },
            },
        );
        assert_eq!(
            db.operand_rules(node_id),
            Arc::new(Rules {
                default: [NodeOperation::UpdateInteger].into_iter().collect(),
                ..Default::default()
            })
        );
    }
}
