use components::{
    event::{Event, EventEntry},
    patch::{AttributePatch, ContentPatch, OperandsPatch},
    rules::{AuditResponse, NodeOperation, SpaceOperation},
    snapshot::Snapshot,
    user::UserId,
};
use deskc_ids::NodeId;

use crate::Kernel;

enum Operation<'a> {
    Space(SpaceOperation),
    Node(NodeOperation, &'a NodeId),
    Both(SpaceOperation, NodeOperation, &'a NodeId),
}

impl Kernel {
    pub fn audit(&self, entry: &EventEntry) -> AuditResponse {
        // even by owners, loop is prohibited
        if self.loop_detector.does_make_loop(&entry.event) {
            return AuditResponse::Denied;
        }
        if self.snapshot.owners.contains(&entry.user_id) {
            return AuditResponse::Allowed;
        }
        use NodeOperation::*;
        use SpaceOperation::*;
        let operation = match &entry.event {
            Event::AddOwner { .. } => {
                // Initial addition of owner is always allowed.
                if self.snapshot.owners.is_empty() {
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
            Event::UpdateParent { node_id, parent } => {
                todo!()
            }
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
            Operation::Space(operation) => self.snapshot.rules.audit(&entry.user_id, &operation),
            Operation::Node(operation, node_id) => {
                audit_node(&self.snapshot, &entry.user_id, node_id, &operation)
            }
            Operation::Both(space_operation, node_operation, node_id) => {
                if self.snapshot.rules.audit(&entry.user_id, &space_operation)
                    == AuditResponse::Denied
                {
                    // space denies
                    return AuditResponse::Denied;
                }
                // audit by node
                audit_node(&self.snapshot, &entry.user_id, node_id, &node_operation)
            }
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
    use std::sync::Arc;

    use components::{content::Content, flat_node::FlatNode, rules::Rules, user::UserId};
    use deskc_hir::expr::{Expr, Literal};
    use deskc_ids::NodeId;
    use deskc_types::Type;

    use crate::{descendants::DescendantsQueries, repository::Repository};

    use super::*;

    pub struct TestRepository {}

    impl Repository for TestRepository {
        fn poll(&mut self) -> Vec<EventEntry> {
            panic!()
        }
        fn commit(&mut self, _log: Event) {
            panic!()
        }
        fn add_owner(&mut self, _user_id: UserId) {
            panic!()
        }
        fn remove_owner(&mut self, _user_id: UserId) {
            panic!()
        }
    }

    #[test]
    fn initial_add_owner_is_always_allowed() {
        let kernel = Kernel::new(TestRepository {});

        assert_eq!(
            kernel.audit(&EventEntry {
                index: 0,
                user_id: UserId("a".into()),
                event: Event::AddOwner {
                    user_id: UserId("a".into()),
                }
            }),
            AuditResponse::Allowed
        );
    }

    #[test]
    fn any_event_allowed_for_owners() {
        let mut kernel = Kernel::new(TestRepository {});
        kernel.snapshot.handle_event(&Event::AddOwner {
            user_id: UserId("a".into()),
        });
        assert_eq!(
            kernel.audit(&EventEntry {
                index: 0,
                user_id: UserId("a".into()),
                event: Event::AddOwner {
                    user_id: UserId("b".into()),
                },
            }),
            AuditResponse::Allowed
        );
        assert_eq!(
            kernel.audit(&EventEntry {
                index: 0,
                user_id: UserId("a".into()),
                event: Event::UpdateSpaceRules {
                    rules: Rules::default()
                }
            }),
            AuditResponse::Allowed
        );
    }

    #[test]
    fn update_space_rule_denied() {
        let mut kernel = Kernel::new(TestRepository {});
        kernel.snapshot.handle_event(&Event::AddOwner {
            user_id: UserId("a".into()),
        });
        assert_eq!(
            kernel.audit(&EventEntry {
                index: 0,
                user_id: UserId("b".into()),
                event: Event::UpdateSpaceRules {
                    rules: Rules::default()
                }
            }),
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
        let mut base = Kernel::new(TestRepository {});
        use NodeOperation::*;
        base.snapshot.flat_nodes.insert(
            node_id_a.clone(),
            FlatNode::new(Content::Integer(1)).rules(Rules {
                default: [RemoveNode, PatchContentReplace].into_iter().collect(),
                users: Default::default(),
            }),
        );
        base.snapshot.flat_nodes.insert(
            node_id_b.clone(),
            FlatNode::new(Content::Integer(1))
                .rules(Rules {
                    default: [RemoveNode, PatchContentAddInteger].into_iter().collect(),
                    users: Default::default(),
                })
                .parent(Some(node_id_a.clone())),
        );

        // Allowed by the node
        let kernel = Kernel {
            snapshot: base.snapshot.clone(),
            ..Kernel::new(TestRepository {})
        };
        assert_eq!(
            kernel.audit(&replace_entry(node_id_a.clone())),
            AuditResponse::Allowed
        );

        // Allowed by both
        let kernel = Kernel {
            snapshot: base.snapshot.clone(),
            ..Kernel::new(TestRepository {})
        };
        assert_eq!(
            kernel.audit(&remove_node_entry(node_id_b.clone())),
            AuditResponse::Allowed
        );

        // Denied by the node
        let kernel = Kernel {
            snapshot: base.snapshot.clone(),
            ..Kernel::new(TestRepository {})
        };
        assert_eq!(
            kernel.audit(&replace_entry(node_id_b.clone())),
            AuditResponse::Denied
        );

        // Denied by the parent
        let kernel = Kernel {
            snapshot: base.snapshot.clone(),
            ..Kernel::new(TestRepository {})
        };
        assert_eq!(
            kernel.audit(&add_integer_entry(node_id_b.clone())),
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
                let kernel = Kernel::new(TestRepository {});
                assert_eq!(kernel.audit(&event_entry), AuditResponse::Denied);

                // Allowed default includes operation
                let mut kernel = Kernel::new(TestRepository {});
                kernel.snapshot.rules.default = [SpaceOperation::$operation].into_iter().collect();
                assert_eq!(kernel.audit(&event_entry), AuditResponse::Allowed);

                // Allowed user includes operation
                let mut kernel = Kernel::new(TestRepository {});
                kernel.snapshot.rules.users.insert(
                    UserId("a".into()),
                    [SpaceOperation::$operation].into_iter().collect(),
                );
                assert_eq!(kernel.audit(&event_entry), AuditResponse::Allowed);
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
                let mut base = Kernel::new(TestRepository {});
                base.snapshot
                    .flat_nodes
                    .insert(node_id.clone(), FlatNode::new(Content::String("a".into())));
                base.loop_detector
                    .operand
                    .lock()
                    .set_node(node_id.clone(), Arc::new(Default::default()));

                // Denied
                let kernel = Kernel {
                    snapshot: base.snapshot.clone(),
                    ..Kernel::new(TestRepository {})
                };
                assert_eq!(kernel.audit(&event_entry), AuditResponse::Denied);

                // Allowed default includes operation
                let mut kernel = Kernel {
                    snapshot: base.snapshot.clone(),
                    ..Kernel::new(TestRepository {})
                };
                kernel
                    .snapshot
                    .flat_nodes
                    .get_mut(&node_id)
                    .unwrap()
                    .rules
                    .default = [NodeOperation::$operation].into_iter().collect();
                assert_eq!(kernel.audit(&event_entry), AuditResponse::Allowed);

                // Allowed user includes operation
                let mut kernel = Kernel {
                    snapshot: base.snapshot.clone(),
                    ..Kernel::new(TestRepository {})
                };
                assert_eq!(kernel.audit(&event_entry), AuditResponse::Denied);
                kernel
                    .snapshot
                    .flat_nodes
                    .get_mut(&node_id)
                    .unwrap()
                    .rules
                    .users
                    .insert(
                        UserId("a".into()),
                        [NodeOperation::$operation].into_iter().collect(),
                    );
                assert_eq!(kernel.audit(&event_entry), AuditResponse::Allowed);
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
                let mut base = Kernel::new(TestRepository {});
                base.snapshot
                    .flat_nodes
                    .insert(node_id.clone(), FlatNode::new(Content::String("a".into())));
                let allow_by_space = |kernel: &mut Kernel| {
                    kernel.snapshot.rules.default =
                        [SpaceOperation::$operation].into_iter().collect();
                };
                let allow_by_node = |kernel: &mut Kernel| {
                    kernel
                        .snapshot
                        .flat_nodes
                        .get_mut(&node_id)
                        .unwrap()
                        .rules
                        .default
                        .insert(NodeOperation::$operation);
                };

                // Denied by both
                let kernel = Kernel {
                    snapshot: base.snapshot.clone(),
                    ..Kernel::new(TestRepository {})
                };
                assert_eq!(kernel.audit(&event_entry), AuditResponse::Denied);

                // Denied by node
                let mut kernel = Kernel {
                    snapshot: base.snapshot.clone(),
                    ..Kernel::new(TestRepository {})
                };
                allow_by_space(&mut kernel);
                assert_eq!(kernel.audit(&event_entry), AuditResponse::Denied);

                // Denied by space
                let mut kernel = Kernel {
                    snapshot: base.snapshot.clone(),
                    ..Kernel::new(TestRepository {})
                };
                allow_by_node(&mut kernel);
                assert_eq!(kernel.audit(&event_entry), AuditResponse::Denied);

                // Allowed
                let mut kernel = Kernel {
                    snapshot: base.snapshot.clone(),
                    ..Kernel::new(TestRepository {})
                };
                allow_by_space(&mut kernel);
                allow_by_node(&mut kernel);
                assert_eq!(kernel.audit(&event_entry), AuditResponse::Allowed);
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
    #[test]
    fn patch_operands_insert() {
        let node_id = NodeId::new();
        let node_operand = NodeId::new();
        let event_entry = EventEntry {
            index: 0,
            user_id: UserId("a".into()),
            event: Event::PatchOperands {
                node_id: node_id.clone(),
                patch: OperandsPatch::Insert {
                    index: 0,
                    node: node_operand.clone(),
                },
            },
        };
        let mut base = Kernel::new(TestRepository {});
        base.snapshot
            .flat_nodes
            .insert(node_id.clone(), FlatNode::new(Content::String("a".into())));

        // Denied
        let kernel = Kernel {
            snapshot: base.snapshot.clone(),
            ..Kernel::new(TestRepository {})
        };
        // Here is the difference from other tests
        kernel
            .loop_detector
            .operand
            .lock()
            .set_node(dbg!(node_operand.clone()), Arc::new(Default::default()));
        assert_eq!(kernel.audit(&event_entry), AuditResponse::Denied);

        // Allowed default includes operation
        let mut kernel = Kernel {
            snapshot: base.snapshot.clone(),
            ..Kernel::new(TestRepository {})
        };
        // Here is the difference from other tests
        kernel
            .loop_detector
            .operand
            .lock()
            .set_node(dbg!(node_operand.clone()), Arc::new(Default::default()));
        kernel
            .snapshot
            .flat_nodes
            .get_mut(&node_id)
            .unwrap()
            .rules
            .default = [NodeOperation::PatchOperandsInsert].into_iter().collect();
        assert_eq!(kernel.audit(&event_entry), AuditResponse::Allowed);

        // Allowed user includes operation
        let mut kernel = Kernel {
            snapshot: base.snapshot.clone(),
            ..Kernel::new(TestRepository {})
        };
        // Here is the difference from other tests
        kernel
            .loop_detector
            .operand
            .lock()
            .set_node(dbg!(node_operand.clone()), Arc::new(Default::default()));
        assert_eq!(kernel.audit(&event_entry), AuditResponse::Denied);
        kernel
            .snapshot
            .flat_nodes
            .get_mut(&node_id)
            .unwrap()
            .rules
            .users
            .insert(
                UserId("a".into()),
                [NodeOperation::PatchOperandsInsert].into_iter().collect(),
            );
        assert_eq!(kernel.audit(&event_entry), AuditResponse::Allowed);
    }
    test_node!(
        patch_operands_remove,
        |node_id| {
            Event::PatchOperands {
                node_id,
                patch: OperandsPatch::Remove { index: 0 },
            }
        },
        PatchOperandsRemove
    );
    test_node!(
        patch_operands_move,
        |node_id| {
            Event::PatchOperands {
                node_id,
                patch: OperandsPatch::Move { index: 0, diff: 0 },
            }
        },
        PatchOperandsMove
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
    fn deny_always_if_node_not_found() {
        let node_id = NodeId::new();
        let snapshot = Snapshot::default();
        assert_eq!(
            audit_node(
                &snapshot,
                &UserId("a".into()),
                &node_id,
                &NodeOperation::RemoveNode
            ),
            AuditResponse::Denied
        );
    }

    #[test]
    fn deny_always_if_parent_not_found_for_adding_new_node() {
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
                &NodeOperation::RemoveNode
            ),
            AuditResponse::Denied
        );
    }

    #[test]
    fn prevent_loop() {
        let node_a = NodeId::new();
        let node_b = NodeId::new();
        let mut kernel = Kernel::new(TestRepository {});
        kernel.snapshot.owners.insert(UserId("a".into()));

        kernel.loop_detector.operand.lock().set_node(
            node_a.clone(),
            Arc::new([node_b.clone()].into_iter().collect()),
        );
        kernel
            .loop_detector
            .operand
            .lock()
            .set_node(node_b.clone(), Arc::new([].into_iter().collect()));
        assert_eq!(
            kernel.audit(&EventEntry {
                index: 0,
                user_id: UserId("a".into()),
                event: Event::PatchOperands {
                    node_id: node_a.clone(),
                    patch: OperandsPatch::Insert {
                        index: 0,
                        node: node_b.clone(),
                    },
                },
            }),
            AuditResponse::Allowed
        );
        assert_eq!(
            kernel.audit(&EventEntry {
                index: 0,
                user_id: UserId("a".into()),
                event: Event::PatchOperands {
                    node_id: node_b,
                    patch: OperandsPatch::Insert {
                        index: 0,
                        node: node_a,
                    },
                },
            }),
            AuditResponse::Denied
        );
    }
}
