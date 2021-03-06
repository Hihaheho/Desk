use components::{
    event::{Event, EventEntry},
    patch::{AttributePatch, ChildrenPatch, ContentPatch, FilePatch},
    rules::{AuditResponse, NodeOperation, SpaceOperation},
    snapshot::Snapshot,
};
use deskc_ids::FileId;

enum Operation<'a> {
    Space(SpaceOperation),
    Node(NodeOperation, &'a FileId),
}

macro_rules! file_id {
    ($snapshot:expr, $node_id:expr) => {
        if let Some(node) = $snapshot.flat_nodes.get($node_id) {
            &node.file_id
        } else {
            return AuditResponse::Denied;
        }
    };
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
        Event::AddNode { file_id, .. } => Operation::Node(AddNode, file_id),
        Event::RemoveNode { node_id } => Operation::Node(RemoveNode, file_id!(snapshot, node_id)),
        Event::PatchContent { node_id, patch } => {
            let operation = match patch {
                ContentPatch::Replace(_) => PatchContentReplace,
                ContentPatch::PatchString(_) => PatchContentPatchString,
                ContentPatch::AddInteger(_) => PatchContentAddInteger,
                ContentPatch::AddFloat(_) => PatchContentAddFloat,
            };
            Operation::Node(operation, file_id!(snapshot, node_id))
        }
        Event::PatchChildren { node_id, patch } => {
            let operation = match patch {
                ChildrenPatch::Insert { .. } => PatchChildrenInsert,
                ChildrenPatch::Remove { .. } => PatchChildrenRemove,
                ChildrenPatch::Move { .. } => PatchChildrenMove,
                ChildrenPatch::Update { .. } => PatchChildrenUpdate,
            };
            Operation::Node(operation, file_id!(snapshot, node_id))
        }
        Event::PatchAttribute { node_id, patch } => {
            let operation = match patch {
                AttributePatch::Update { .. } => PatchAttributeUpdate,
                AttributePatch::Remove { .. } => PatchAttributeRemove,
            };
            Operation::Node(operation, file_id!(snapshot, node_id))
        }
        Event::AddSnapshot { .. } => Operation::Space(AddSnapshot),
        Event::AddFile(_) => Operation::Space(AddFile),
        Event::DeleteFile(_) => Operation::Space(DeleteFile),
        Event::UpdateRule { .. } => Operation::Space(UpdateRule),
        Event::PatchFile { file_id, patch } => {
            let operation = match patch {
                FilePatch::UpdateRules { .. } => PatchFileUpdateRules,
            };
            Operation::Node(operation, file_id)
        }
    };
    match operation {
        Operation::Space(operation) => snapshot.rules.audit(&entry.user_id, &operation),
        Operation::Node(operation, file_id) => {
            if let Some(file) = snapshot.files.get(file_id) {
                file.rules.audit(&entry.user_id, &operation)
            } else {
                AuditResponse::Denied
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use components::{content::Content, file::File, flat_node::FlatNode, user::UserId};
    use deskc_hir::expr::{Expr, Literal};
    use deskc_ids::{FileId, NodeId};
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
        // `()` indicates that the macro takes no argument.
        ($fn:ident, $event:expr, $operation:ident) => {
            #[test]
            fn $fn() {
                let node_id = NodeId::new();
                let file_id = FileId::new();
                let event_entry = EventEntry {
                    index: 0,
                    user_id: UserId("a".into()),
                    event: $event(node_id.clone(), file_id.clone()),
                };
                let mut base = Snapshot::default();
                base.files.insert(file_id.clone(), File::default());
                base.flat_nodes.insert(
                    node_id.clone(),
                    FlatNode {
                        file_id: file_id.clone(),
                        content: Content::String("a".into()),
                        children: Default::default(),
                        attributes: Default::default(),
                    },
                );

                // Denied
                let snapshot = base.clone();
                assert_eq!(audit(&snapshot, &event_entry), AuditResponse::Denied);

                // Allowed default includes operation
                let mut snapshot = base.clone();
                snapshot.files.get_mut(&file_id).unwrap().rules.default =
                    [NodeOperation::$operation].into_iter().collect();
                assert_eq!(audit(&snapshot, &event_entry), AuditResponse::Allowed);

                // Allowed user includes operation
                let mut snapshot = base.clone();
                snapshot
                    .files
                    .get_mut(&file_id)
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
    test_space!(add_file, Event::AddFile(FileId::new()), AddFile);
    test_space!(delete_file, Event::DeleteFile(FileId::new()), DeleteFile);
    test_space!(
        update_rule,
        Event::UpdateRule {
            rules: Default::default()
        },
        UpdateRule
    );

    test_node!(
        add_node,
        |node_id, file_id| Event::AddNode {
            node_id,
            file_id,
            content: Content::String("a".into())
        },
        AddNode
    );
    test_node!(
        remove_node,
        |node_id, _| Event::RemoveNode { node_id },
        RemoveNode
    );
    test_node!(
        patch_content_replace,
        |node_id, _| {
            Event::PatchContent {
                node_id,
                patch: ContentPatch::Replace(Content::String("a".into())),
            }
        },
        PatchContentReplace
    );
    test_node!(
        patch_content_patch_string,
        |node_id, _| {
            Event::PatchContent {
                node_id,
                patch: ContentPatch::PatchString(vec![]),
            }
        },
        PatchContentPatchString
    );
    test_node!(
        patch_content_add_integer,
        |node_id, _| {
            Event::PatchContent {
                node_id,
                patch: ContentPatch::AddInteger(0),
            }
        },
        PatchContentAddInteger
    );
    test_node!(
        patch_content_add_float,
        |node_id, _| {
            Event::PatchContent {
                node_id,
                patch: ContentPatch::AddFloat(0.0),
            }
        },
        PatchContentAddFloat
    );
    test_node!(
        patch_children_insert,
        |node_id, _| Event::PatchChildren {
            node_id,
            patch: ChildrenPatch::Insert {
                index: 0,
                node: NodeId::new(),
            }
        },
        PatchChildrenInsert
    );
    test_node!(
        patch_children_remove,
        |node_id, _| {
            Event::PatchChildren {
                node_id,
                patch: ChildrenPatch::Remove { index: 0 },
            }
        },
        PatchChildrenRemove
    );
    test_node!(
        patch_children_move,
        |node_id, _| {
            Event::PatchChildren {
                node_id,
                patch: ChildrenPatch::Move { index: 0, diff: 0 },
            }
        },
        PatchChildrenMove
    );
    test_node!(
        patch_children_update,
        |node_id, _| {
            Event::PatchChildren {
                node_id,
                patch: ChildrenPatch::Update {
                    index: 0,
                    node: NodeId::new(),
                },
            }
        },
        PatchChildrenUpdate
    );
    test_node!(
        patch_attribute_update,
        |node_id, _| {
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
        |node_id, _| {
            Event::PatchAttribute {
                node_id,
                patch: AttributePatch::Remove { key: Type::Number },
            }
        },
        PatchAttributeRemove
    );
    test_node!(
        patch_file_update_rules,
        |_, file_id| {
            Event::PatchFile {
                file_id,
                patch: FilePatch::UpdateRules {
                    rules: Default::default(),
                },
            }
        },
        PatchFileUpdateRules
    );
}
