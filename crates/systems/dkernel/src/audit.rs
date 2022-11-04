use components::{
    event::{Event, EventEntry},
    patch::{AttributePatch, ContentPatch, OperandsPatch},
    rules::{AuditResponse, NodeOperation, SpaceOperation},
    snapshot::Snapshot,
    user::UserId,
};
use deskc_ids::NodeId;

enum Operation<'a> {
    Space(SpaceOperation),
    Node(NodeOperation, &'a NodeId),
    Both(SpaceOperation, NodeOperation, &'a NodeId),
}

pub fn audit(snapshot: &Snapshot, entry: &EventEntry) -> AuditResponse {
    if snapshot.owners.contains(&entry.user_id) {
        return AuditResponse::Allowed;
    }
    use NodeOperation::*;
    use SpaceOperation::*;
    let operation = match &entry.event {
        Event::AddOwner { .. } => {
            // Initial addition of owner is always allowed.
            if snapshot.owners.is_empty() {
                return AuditResponse::Allowed;
            }
            Operation::Space(AddOwner)
        }
        Event::RemoveOwner { .. } => Operation::Space(RemoveOwner),
        Event::AddNode { parent, .. } => {
            if let Some(parent) = parent {
                Operation::Both(SpaceOperation::AddNode, NodeOperation::AddNode, parent)
            } else {
                Operation::Space(SpaceOperation::AddNode)
            }
        }
        Event::RemoveNode { node_id } => Operation::Node(RemoveNode, node_id),
        Event::PatchContent { node_id, patch } => {
            let operation = match patch {
                ContentPatch::Replace(_) => PatchContentReplace,
                ContentPatch::PatchString(_) => PatchContentPatchString,
                ContentPatch::AddInteger(_) => PatchContentAddInteger,
                ContentPatch::AddFloat(_) => PatchContentAddFloat,
            };
            Operation::Node(operation, node_id)
        }
        Event::PatchOperands { node_id, patch } => {
            let operation = match patch {
                OperandsPatch::Insert { .. } => PatchOperandsInsert,
                OperandsPatch::Remove { .. } => PatchOperandsRemove,
                OperandsPatch::Move { .. } => PatchOperandsMove,
                OperandsPatch::Update { .. } => PatchOperandsUpdate,
            };
            Operation::Node(operation, node_id)
        }
        Event::PatchAttribute { node_id, patch } => {
            let operation = match patch {
                AttributePatch::Update { .. } => PatchAttributeUpdate,
                AttributePatch::Remove { .. } => PatchAttributeRemove,
            };
            Operation::Node(operation, node_id)
        }
        Event::AddSnapshot { .. } => Operation::Space(AddSnapshot),
        Event::UpdateSpaceRules { .. } => {
            return AuditResponse::Denied;
        }
        Event::UpdateNodeRules { node_id, .. } => Operation::Node(UpdateRules, node_id),
    };
    match operation {
        Operation::Space(operation) => snapshot.rules.audit(&entry.user_id, &operation),
        Operation::Node(operation, node_id) => {
            audit_node(snapshot, &entry.user_id, node_id, &operation)
        }
        Operation::Both(space_operation, node_operation, node_id) => {
            if snapshot.rules.audit(&entry.user_id, &space_operation) == AuditResponse::Denied {
                // space denies
                return AuditResponse::Denied;
            }
            // audit by node
            audit_node(snapshot, &entry.user_id, node_id, &node_operation)
        }
    }
}

fn audit_node(
    snapshot: &Snapshot,
    user_id: &UserId,
    node_id: &NodeId,
    operation: &NodeOperation,
) -> AuditResponse {
    if let Some(mut flat_node) = dbg!(snapshot.flat_nodes.get(node_id)) {
        let mut rule = flat_node.rules.clone();
        while let Some(parent_id) = dbg!(&flat_node.parent) {
            if let Some(parent) = snapshot.flat_nodes.get(parent_id) {
                flat_node = parent;
                rule = rule.intersection(&flat_node.rules);
            } else {
                return AuditResponse::Denied;
            }
        }
        rule.audit(user_id, operation)
    } else {
        AuditResponse::Denied
    }
}

#[cfg(test)]
mod tests {
    use components::{content::Content, flat_node::FlatNode, rules::Rules, user::UserId};
    use deskc_hir::expr::{Expr, Literal};
    use deskc_ids::NodeId;
    use deskc_types::Type;

    use super::*;

    #[test]
    fn initial_add_owner_is_always_allowed() {
        let snapshot = Snapshot::default();
        assert_eq!(
            audit(
                &snapshot,
                &EventEntry {
                    index: 0,
                    user_id: UserId("a".into()),
                    event: Event::AddOwner {
                        user_id: UserId("a".into()),
                    }
                }
            ),
            AuditResponse::Allowed
        );
    }

    #[test]
    fn any_event_allowed_for_owners() {
        let mut snapshot = Snapshot::default();
        snapshot.handle_event(&Event::AddOwner {
            user_id: UserId("a".into()),
        });
        assert_eq!(
            audit(
                &snapshot,
                &EventEntry {
                    index: 0,
                    user_id: UserId("a".into()),
                    event: Event::AddOwner {
                        user_id: UserId("b".into()),
                    },
                }
            ),
            AuditResponse::Allowed
        );
        assert_eq!(
            audit(
                &snapshot,
                &EventEntry {
                    index: 0,
                    user_id: UserId("a".into()),
                    event: Event::UpdateSpaceRules {
                        rules: Rules::default()
                    }
                }
            ),
            AuditResponse::Allowed
        );
    }

    #[test]
    fn update_space_rule_denied() {
        let mut snapshot = Snapshot::default();
        snapshot.handle_event(&Event::AddOwner {
            user_id: UserId("a".into()),
        });
        assert_eq!(
            audit(
                &snapshot,
                &EventEntry {
                    index: 0,
                    user_id: UserId("b".into()),
                    event: Event::UpdateSpaceRules {
                        rules: Rules::default()
                    }
                }
            ),
            AuditResponse::Denied
        );
    }

    #[test]
    fn hierarchy() {
        let node_id_a = NodeId::new();
        let node_id_b = NodeId::new();
        let replace_entry = |node_id: NodeId| EventEntry {
            index: 0,
            user_id: UserId("a".into()),
            event: Event::PatchContent {
                node_id: node_id.clone(),
                patch: ContentPatch::Replace(Content::Integer(1)),
            },
        };
        let add_integer_entry = |node_id: NodeId| EventEntry {
            index: 0,
            user_id: UserId("a".into()),
            event: Event::PatchContent {
                node_id: node_id.clone(),
                patch: ContentPatch::AddInteger(2),
            },
        };
        let remove_node_entry = |node_id: NodeId| EventEntry {
            index: 0,
            user_id: UserId("a".into()),
            event: Event::RemoveNode {
                node_id: node_id.clone(),
            },
        };
        let mut base = Snapshot::default();
        use NodeOperation::*;
        base.flat_nodes.insert(
            node_id_a.clone(),
            FlatNode::new(Content::Integer(1)).rules(Rules {
                default: [RemoveNode, PatchContentReplace].into_iter().collect(),
                users: Default::default(),
            }),
        );
        base.flat_nodes.insert(
            node_id_b.clone(),
            FlatNode::new(Content::Integer(1))
                .rules(Rules {
                    default: [RemoveNode, PatchContentAddInteger].into_iter().collect(),
                    users: Default::default(),
                })
                .parent(Some(node_id_a.clone())),
        );

        // Allowed by the node
        let snapshot = base.clone();
        assert_eq!(
            audit(&snapshot, &replace_entry(node_id_a.clone())),
            AuditResponse::Allowed
        );

        // Allowed by both
        let snapshot = base.clone();
        assert_eq!(
            audit(&snapshot, &remove_node_entry(node_id_b.clone())),
            AuditResponse::Allowed
        );

        // Denied by the node
        let snapshot = base.clone();
        assert_eq!(
            audit(&snapshot, &replace_entry(node_id_b.clone())),
            AuditResponse::Denied
        );

        // Denied by the parent
        let snapshot = base.clone();
        assert_eq!(
            audit(&snapshot, &add_integer_entry(node_id_b.clone())),
            AuditResponse::Denied
        );
    }

    macro_rules! test_space {
        // `()` indicates that the macro takes no argument.
        ($fn:ident, $event:expr, $operation:ident) => {
            #[test]
            fn $fn() {
                let event_entry = EventEntry {
                    index: 0,
                    user_id: UserId("a".into()),
                    event: $event,
                };
                // Denied
                let snapshot = Snapshot::default();
                assert_eq!(audit(&snapshot, &event_entry), AuditResponse::Denied);

                // Allowed default includes operation
                let mut snapshot = Snapshot::default();
                snapshot.rules.default = [SpaceOperation::$operation].into_iter().collect();
                assert_eq!(audit(&snapshot, &event_entry), AuditResponse::Allowed);

                // Allowed user includes operation
                let mut snapshot = Snapshot::default();
                snapshot.rules.users.insert(
                    UserId("a".into()),
                    [SpaceOperation::$operation].into_iter().collect(),
                );
                assert_eq!(audit(&snapshot, &event_entry), AuditResponse::Allowed);
            }
        };
    }

    macro_rules! test_node {
        ($fn:ident, $event:expr, $operation:ident) => {
            #[test]
            fn $fn() {
                let node_id = NodeId::new();
                let event_entry = EventEntry {
                    index: 0,
                    user_id: UserId("a".into()),
                    event: $event(node_id.clone()),
                };
                let mut base = Snapshot::default();
                base.flat_nodes
                    .insert(node_id.clone(), FlatNode::new(Content::String("a".into())));

                // Denied
                let snapshot = base.clone();
                assert_eq!(audit(&snapshot, &event_entry), AuditResponse::Denied);

                // Allowed default includes operation
                let mut snapshot = base.clone();
                snapshot.flat_nodes.get_mut(&node_id).unwrap().rules.default =
                    [NodeOperation::$operation].into_iter().collect();
                assert_eq!(audit(&snapshot, &event_entry), AuditResponse::Allowed);

                // Allowed user includes operation
                let mut snapshot = base.clone();
                assert_eq!(audit(&snapshot, &event_entry), AuditResponse::Denied);
                snapshot
                    .flat_nodes
                    .get_mut(&node_id)
                    .unwrap()
                    .rules
                    .users
                    .insert(
                        UserId("a".into()),
                        [NodeOperation::$operation].into_iter().collect(),
                    );
                assert_eq!(audit(&snapshot, &event_entry), AuditResponse::Allowed);
            }
        };
    }

    macro_rules! test_both {
        ($fn:ident, $event:expr, $operation:ident) => {
            #[test]
            fn $fn() {
                let node_id = NodeId::new();
                let event_entry = EventEntry {
                    index: 0,
                    user_id: UserId("a".into()),
                    event: $event(node_id.clone()),
                };
                let mut base = Snapshot::default();
                base.flat_nodes
                    .insert(node_id.clone(), FlatNode::new(Content::String("a".into())));
                let allow_by_space = |snapshot: &mut Snapshot| {
                    snapshot.rules.default = [SpaceOperation::$operation].into_iter().collect();
                };
                let allow_by_node = |snapshot: &mut Snapshot| {
                    snapshot
                        .flat_nodes
                        .get_mut(&node_id)
                        .unwrap()
                        .rules
                        .default
                        .insert(NodeOperation::$operation);
                };

                // Denied by both
                let snapshot = base.clone();
                assert_eq!(audit(&snapshot, &event_entry), AuditResponse::Denied);

                // Denied by node
                let mut snapshot = base.clone();
                allow_by_space(&mut snapshot);
                assert_eq!(audit(&snapshot, &event_entry), AuditResponse::Denied);

                // Denied by space
                let mut snapshot = base.clone();
                allow_by_node(&mut snapshot);
                assert_eq!(audit(&snapshot, &event_entry), AuditResponse::Denied);

                // Allowed
                let mut snapshot = base.clone();
                allow_by_space(&mut snapshot);
                allow_by_node(&mut snapshot);
                assert_eq!(audit(&snapshot, &event_entry), AuditResponse::Allowed);
            }
        };
    }

    test_space!(
        remove_owner,
        Event::RemoveOwner {
            user_id: UserId("a".into())
        },
        RemoveOwner
    );
    test_space!(
        add_snapshot,
        Event::AddSnapshot {
            index: 0,
            snapshot: Default::default()
        },
        AddSnapshot
    );
    test_space!(
        add_node_space,
        Event::AddNode {
            parent: None,
            node_id: NodeId::new(),
            content: Content::Integer(1)
        },
        AddNode
    );

    test_both!(
        add_node_both,
        |node_id| Event::AddNode {
            parent: Some(node_id),
            node_id: NodeId::new(),
            content: Content::String("a".into())
        },
        AddNode
    );

    test_node!(
        remove_node,
        |node_id| Event::RemoveNode { node_id },
        RemoveNode
    );
    test_node!(
        patch_content_replace,
        |node_id| {
            Event::PatchContent {
                node_id,
                patch: ContentPatch::Replace(Content::String("a".into())),
            }
        },
        PatchContentReplace
    );
    test_node!(
        patch_content_patch_string,
        |node_id| {
            Event::PatchContent {
                node_id,
                patch: ContentPatch::PatchString(vec![]),
            }
        },
        PatchContentPatchString
    );
    test_node!(
        patch_content_add_integer,
        |node_id| {
            Event::PatchContent {
                node_id,
                patch: ContentPatch::AddInteger(0),
            }
        },
        PatchContentAddInteger
    );
    test_node!(
        patch_content_add_float,
        |node_id| {
            Event::PatchContent {
                node_id,
                patch: ContentPatch::AddFloat(0.0),
            }
        },
        PatchContentAddFloat
    );
    test_node!(
        patch_children_insert,
        |node_id| Event::PatchOperands {
            node_id,
            patch: OperandsPatch::Insert {
                index: 0,
                node: NodeId::new(),
            }
        },
        PatchOperandsInsert
    );
    test_node!(
        patch_children_remove,
        |node_id| {
            Event::PatchOperands {
                node_id,
                patch: OperandsPatch::Remove { index: 0 },
            }
        },
        PatchOperandsRemove
    );
    test_node!(
        patch_children_move,
        |node_id| {
            Event::PatchOperands {
                node_id,
                patch: OperandsPatch::Move { index: 0, diff: 0 },
            }
        },
        PatchOperandsMove
    );
    test_node!(
        patch_children_update,
        |node_id| {
            Event::PatchOperands {
                node_id,
                patch: OperandsPatch::Update {
                    index: 0,
                    node: NodeId::new(),
                },
            }
        },
        PatchOperandsUpdate
    );
    test_node!(
        patch_attribute_update,
        |node_id| {
            Event::PatchAttribute {
                node_id,
                patch: AttributePatch::Update {
                    key: Type::Number,
                    value: Box::new(Expr::Literal(Literal::Integer(0))),
                },
            }
        },
        PatchAttributeUpdate
    );
    test_node!(
        patch_attribute_remove,
        |node_id| {
            Event::PatchAttribute {
                node_id,
                patch: AttributePatch::Remove { key: Type::Number },
            }
        },
        PatchAttributeRemove
    );

    #[test]
    fn node_not_found() {
        let node_id = NodeId::new();
        let snapshot = Snapshot::default();
        assert_eq!(
            audit_node(
                &snapshot,
                &UserId("a".into()),
                &node_id,
                &NodeOperation::AddNode
            ),
            AuditResponse::Denied
        );
    }

    #[test]
    fn parent_not_fount() {
        let node_id = NodeId::new();
        let mut snapshot = Snapshot::default();
        snapshot.flat_nodes.insert(
            node_id.clone(),
            FlatNode::new(Content::String("a".into())).parent(Some(NodeId::new())),
        );
        assert_eq!(
            audit_node(
                &snapshot,
                &UserId("a".into()),
                &node_id,
                &NodeOperation::AddNode
            ),
            AuditResponse::Denied
        );
    }
}
