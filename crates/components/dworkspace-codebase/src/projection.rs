use std::collections::{HashMap, HashSet};

use crate::event::{Event, EventPayload};
use crate::flat_node::FlatNode;
use crate::rules::{Rules, SpaceOperation};
use crate::user::UserId;
use deskc_ids::NodeId;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Projection {
    pub owners: HashSet<UserId>,
    // flat nodes are owned by hirs db
    pub flat_nodes: HashMap<NodeId, FlatNode>,
    pub rules: Rules<SpaceOperation>,
}

impl Projection {
    pub fn handle_event(&mut self, event: &Event) {
        match &event.payload {
            EventPayload::AddOwner { user_id } => {
                self.owners.insert(user_id.clone());
            }
            EventPayload::RemoveOwner { user_id: _ } => todo!(),
            EventPayload::CreateNode { node_id, content } => {
                self.flat_nodes
                    .insert(node_id.clone(), FlatNode::new(content.clone()));
            }
            EventPayload::RemoveNode { node_id } => {
                self.flat_nodes.remove(node_id);
            }
            EventPayload::PatchContent { node_id, patch } => {
                self.flat_nodes
                    .get_mut(node_id)
                    .unwrap()
                    .patch_content(patch);
            }
            EventPayload::PatchOperand { node_id, patch } => {
                self.flat_nodes
                    .get_mut(node_id)
                    .unwrap()
                    .patch_children(patch);
            }
            EventPayload::PatchAttribute { node_id, patch } => {
                self.flat_nodes
                    .get_mut(node_id)
                    .unwrap()
                    .patch_attribute(patch);
            }
            EventPayload::AddSnapshot {
                index: _,
                snapshot: _,
            } => todo!(),
            EventPayload::UpdateSpaceRules { rules } => {
                self.rules = rules.clone();
            }
            EventPayload::UpdateOperandRules { node_id, rules } => {
                let node = self.flat_nodes.get_mut(node_id).unwrap();
                node.operand_rules = rules.clone();
            }
            EventPayload::UpdateNodeRules { node_id, rules } => {
                let node = self.flat_nodes.get_mut(node_id).unwrap();
                node.rules = rules.clone();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{content::Content, event::EventId, rules::NodeOperation};

    use super::*;

    #[test]
    fn add_owner() {
        let mut snapshot = Projection::default();
        let user_id = UserId::new();
        snapshot.handle_event(&Event {
            id: EventId::new(),
            user_id,
            payload: EventPayload::AddOwner { user_id },
        });
        assert_eq!(snapshot.owners, vec![user_id].into_iter().collect())
    }

    #[test]
    fn add_node() {
        let mut snapshot = Projection::default();
        let node_id = handle_add_node(&mut snapshot);
        assert_eq!(
            snapshot.flat_nodes,
            [(node_id, FlatNode::new(Content::String("a".into())))]
                .into_iter()
                .collect()
        )
    }

    #[test]
    fn remove_node() {
        let mut snapshot = Projection::default();
        let node_id = handle_add_node(&mut snapshot);
        snapshot.handle_event(&Event {
            id: EventId::new(),
            user_id: UserId::new(),
            payload: EventPayload::RemoveNode { node_id },
        });

        assert_eq!(snapshot.flat_nodes, HashMap::default())
    }

    #[test]
    fn update_space_rule() {
        let mut snapshot = Projection::default();
        snapshot.handle_event(&Event {
            id: EventId::new(),
            user_id: UserId::new(),
            payload: EventPayload::UpdateSpaceRules {
                rules: Rules {
                    default: [SpaceOperation::AddSnapshot].into_iter().collect(),
                    users: Default::default(),
                },
            },
        });

        assert_eq!(
            snapshot.rules,
            Rules {
                default: [SpaceOperation::AddSnapshot].into_iter().collect(),
                users: Default::default(),
            }
        );
    }

    #[test]
    fn update_node_rule() {
        let mut snapshot = Projection::default();
        let node_id = handle_add_node(&mut snapshot);
        snapshot.handle_event(&Event {
            id: EventId::new(),
            user_id: UserId::new(),
            payload: EventPayload::UpdateNodeRules {
                node_id: node_id.clone(),
                rules: Rules {
                    default: [NodeOperation::UpdateInteger].into_iter().collect(),
                    users: Default::default(),
                },
            },
        });

        assert_eq!(
            snapshot.flat_nodes.get(&node_id).unwrap().rules,
            Rules {
                default: [NodeOperation::UpdateInteger].into_iter().collect(),
                users: Default::default(),
            }
        );
    }

    #[test]
    fn update_operand_rules() {
        let mut snapshot = Projection::default();
        let node_id = handle_add_node(&mut snapshot);
        snapshot.handle_event(&Event {
            id: EventId::new(),
            user_id: UserId::new(),
            payload: EventPayload::UpdateOperandRules {
                node_id: node_id.clone(),
                rules: Rules {
                    default: [NodeOperation::UpdateInteger].into_iter().collect(),
                    users: Default::default(),
                },
            },
        });

        assert_eq!(
            snapshot.flat_nodes.get(&node_id).unwrap().operand_rules,
            Rules {
                default: [NodeOperation::UpdateInteger].into_iter().collect(),
                users: Default::default(),
            }
        );
    }

    fn handle_add_node(snapshot: &mut Projection) -> NodeId {
        let node_id = NodeId::new();
        let event = Event {
            id: EventId::new(),
            user_id: UserId::new(),
            payload: EventPayload::CreateNode {
                node_id: node_id.clone(),
                content: Content::String("a".into()),
            },
        };
        snapshot.handle_event(&event);
        node_id
    }
}
