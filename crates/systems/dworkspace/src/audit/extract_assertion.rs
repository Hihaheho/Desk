use components::{
    content::ContentKind,
    event::Event,
    patch::{AttributePatch, ContentPatch, OperandPatch},
    rules::{NodeOperation, SpaceOperation},
};

use super::assertion::Assertion;

pub fn extract_assertion(event: &Event) -> Assertion {
    use NodeOperation::*;
    use SpaceOperation::*;
    match event {
        Event::AddOwner { .. } => Assertion::Any(vec![
            Assertion::NoOwner,
            Assertion::Owner,
            Assertion::SpaceAllows(AddOwner),
        ]),
        Event::RemoveOwner { .. } => Assertion::SpaceAllows(RemoveOwner),
        Event::CreateNode { .. } => Assertion::Any(vec![
            Assertion::Owner,
            Assertion::SpaceAllows(SpaceOperation::CreateNode),
        ]),
        Event::RemoveNode { node_id } => Assertion::All(vec![
            Assertion::NodeExists(node_id),
            Assertion::NotReferenced(node_id),
            Assertion::Any(vec![
                Assertion::Owner,
                Assertion::NodeAllows {
                    operation: RemoveNode,
                    node_id,
                },
            ]),
        ]),
        Event::PatchContent { node_id, patch } => {
            use ContentKind::*;
            let (kind, operation) = match patch {
                ContentPatch::Replace(_) => {
                    return Assertion::All(vec![
                        Assertion::NodeExists(node_id),
                        Assertion::Any(vec![
                            Assertion::Owner,
                            Assertion::NodeAllows {
                                operation: ReplaceContent,
                                node_id,
                            },
                        ]),
                    ])
                }
                ContentPatch::ChangeSourceCodeSyntax { .. } => (SourceCode, ChangeSourceCodeSyntax),
                ContentPatch::PatchSourceCode(_) => (SourceCode, PatchSourceCode),
                ContentPatch::PatchString(_) => (String, PatchString),
                ContentPatch::UpdateInteger(_) => (Integer, UpdateInteger),
                ContentPatch::UpdateReal(_) => (Real, UpdateReal),
                ContentPatch::UpdateRational(_, _) => (Rational, UpdateRational),
                ContentPatch::UpdateApply { .. } => (Apply, UpdateApply),
            };
            Assertion::All(vec![
                Assertion::NodeExists(node_id),
                Assertion::ContentKind { node_id, kind },
                Assertion::Any(vec![
                    Assertion::Owner,
                    Assertion::NodeAllows { operation, node_id },
                ]),
            ])
        }
        Event::PatchOperand { node_id, patch } => match patch {
            OperandPatch::Insert {
                index,
                node_id: operand_id,
            } => Assertion::All(vec![
                Assertion::NodeExists(node_id),
                Assertion::NotReferenced(operand_id),
                Assertion::NoOperandLoop {
                    node_id,
                    operand_id,
                },
                Assertion::OperandsHasSize {
                    node_id,
                    size: *index,
                },
                Assertion::Any(vec![
                    Assertion::Owner,
                    Assertion::NodeAllows {
                        operation: InsertOperand,
                        node_id,
                    },
                ]),
            ]),
            OperandPatch::Remove { index } => Assertion::All(vec![
                Assertion::NodeExists(node_id),
                Assertion::OperandsHasSize {
                    node_id,
                    size: index + 1,
                },
                Assertion::Any(vec![
                    Assertion::Owner,
                    Assertion::NodeAllows {
                        operation: RemoveOperand,
                        node_id,
                    },
                ]),
            ]),
            OperandPatch::Move {
                from: index,
                to: next,
            } => Assertion::All(vec![
                Assertion::NodeExists(node_id),
                Assertion::OperandsHasSize {
                    node_id,
                    size: index.max(next) + 1,
                },
                Assertion::Any(vec![
                    Assertion::Owner,
                    Assertion::NodeAllows {
                        operation: MoveOperand,
                        node_id,
                    },
                ]),
            ]),
        },
        Event::PatchAttribute { node_id, patch } => {
            let operation = match patch {
                AttributePatch::Update { key, value: _ } => UpdateAttribute(key.clone()),
                AttributePatch::Remove { key } => RemoveAttribute(key.clone()),
            };
            Assertion::All(vec![
                Assertion::NodeExists(node_id),
                Assertion::Any(vec![
                    Assertion::Owner,
                    Assertion::NodeAllows { operation, node_id },
                ]),
            ])
        }
        Event::AddSnapshot { .. } => {
            Assertion::Any(vec![Assertion::Owner, Assertion::SpaceAllows(AddSnapshot)])
        }
        Event::UpdateSpaceRules { rules: _ } => Assertion::Owner,
        Event::UpdateNodeRules { node_id, rules: _ } => Assertion::All(vec![
            Assertion::NodeExists(node_id),
            Assertion::Any(vec![
                Assertion::Owner,
                Assertion::NodeAllows {
                    operation: UpdateRules,
                    node_id,
                },
            ]),
        ]),
        Event::UpdateOperandRules { node_id, rules: _ } => Assertion::All(vec![
            Assertion::NodeExists(node_id),
            Assertion::Any(vec![
                Assertion::Owner,
                Assertion::NodeAllows {
                    operation: UpdateOperandRules,
                    node_id,
                },
            ]),
        ]),
    }
}

#[cfg(test)]
mod tests {
    use components::{
        code::SyntaxKind,
        content::{Content, ContentKind},
        patch::StringPatch,
        rules::Rules,
        user::UserId,
    };
    use deskc_hir::expr::{Expr, Literal};
    use deskc_ids::{LinkName, NodeId};
    use deskc_ty::Type;

    use super::*;

    #[test]
    fn extract_assertion_for_add_owner() {
        let event = Event::AddOwner {
            user_id: UserId("owner_id".into()),
        };
        assert_eq!(
            extract_assertion(&event),
            Assertion::Any(vec![
                Assertion::NoOwner,
                Assertion::Owner,
                Assertion::SpaceAllows(SpaceOperation::AddOwner),
            ])
        );
    }

    #[test]
    fn extract_assertion_for_remove_owner() {
        let event = Event::RemoveOwner {
            user_id: UserId("owner_id".into()),
        };
        assert_eq!(
            extract_assertion(&event),
            Assertion::SpaceAllows(SpaceOperation::RemoveOwner)
        );
    }

    #[test]
    fn extract_assertion_for_create_node() {
        let event = Event::CreateNode {
            node_id: NodeId::new(),
            content: Content::Integer(1),
        };
        assert_eq!(
            extract_assertion(&event),
            Assertion::Any(vec![
                Assertion::Owner,
                Assertion::SpaceAllows(SpaceOperation::CreateNode),
            ])
        );
    }

    #[test]
    fn extract_assertion_for_remove_node() {
        let node_id = NodeId::new();
        let event = Event::RemoveNode {
            node_id: node_id.clone(),
        };
        assert_eq!(
            extract_assertion(&event),
            Assertion::All(vec![
                Assertion::NodeExists(&node_id),
                Assertion::NotReferenced(&node_id),
                Assertion::Any(vec![
                    Assertion::Owner,
                    Assertion::NodeAllows {
                        node_id: &node_id,
                        operation: NodeOperation::RemoveNode,
                    },
                ]),
            ])
        );
    }

    #[test]
    fn extract_assertion_for_patch_content_replace() {
        let node_id = NodeId::new();
        let event = Event::PatchContent {
            node_id: node_id.clone(),
            patch: ContentPatch::Replace(Content::Integer(1)),
        };
        assert_eq!(
            extract_assertion(&event),
            Assertion::All(vec![
                Assertion::NodeExists(&node_id),
                Assertion::Any(vec![
                    Assertion::Owner,
                    Assertion::NodeAllows {
                        node_id: &node_id,
                        operation: NodeOperation::ReplaceContent,
                    },
                ]),
            ])
        );
    }

    #[test]
    fn extract_assertion_for_patch_source_code() {
        let node_id = NodeId::new();
        let event = Event::PatchContent {
            node_id: node_id.clone(),
            patch: ContentPatch::PatchSourceCode(StringPatch::Replace("1".into())),
        };
        assert_eq!(
            extract_assertion(&event),
            Assertion::All(vec![
                Assertion::NodeExists(&node_id),
                Assertion::ContentKind {
                    node_id: &node_id,
                    kind: ContentKind::SourceCode,
                },
                Assertion::Any(vec![
                    Assertion::Owner,
                    Assertion::NodeAllows {
                        node_id: &node_id,
                        operation: NodeOperation::PatchSourceCode,
                    },
                ]),
            ])
        );
    }

    #[test]
    fn extract_assertion_for_change_source_code_syntax() {
        let node_id = NodeId::new();
        let event = Event::PatchContent {
            node_id: node_id.clone(),
            patch: ContentPatch::ChangeSourceCodeSyntax {
                syntax: SyntaxKind::Minimalist,
                source: "1".into(),
            },
        };
        assert_eq!(
            extract_assertion(&event),
            Assertion::All(vec![
                Assertion::NodeExists(&node_id),
                Assertion::ContentKind {
                    node_id: &node_id,
                    kind: ContentKind::SourceCode,
                },
                Assertion::Any(vec![
                    Assertion::Owner,
                    Assertion::NodeAllows {
                        node_id: &node_id,
                        operation: NodeOperation::ChangeSourceCodeSyntax,
                    },
                ]),
            ])
        );
    }

    #[test]
    fn extract_assertion_for_patch_string() {
        let node_id = NodeId::new();
        let event = Event::PatchContent {
            node_id: node_id.clone(),
            patch: ContentPatch::PatchString(StringPatch::Replace("a".into())),
        };
        assert_eq!(
            extract_assertion(&event),
            Assertion::All(vec![
                Assertion::NodeExists(&node_id),
                Assertion::ContentKind {
                    node_id: &node_id,
                    kind: ContentKind::String,
                },
                Assertion::Any(vec![
                    Assertion::Owner,
                    Assertion::NodeAllows {
                        node_id: &node_id,
                        operation: NodeOperation::PatchString,
                    },
                ]),
            ])
        );
    }

    #[test]
    fn extract_assertion_for_update_integer() {
        let node_id = NodeId::new();
        let event = Event::PatchContent {
            node_id: node_id.clone(),
            patch: ContentPatch::UpdateInteger(1),
        };
        assert_eq!(
            extract_assertion(&event),
            Assertion::All(vec![
                Assertion::NodeExists(&node_id),
                Assertion::ContentKind {
                    node_id: &node_id,
                    kind: ContentKind::Integer,
                },
                Assertion::Any(vec![
                    Assertion::Owner,
                    Assertion::NodeAllows {
                        node_id: &node_id,
                        operation: NodeOperation::UpdateInteger,
                    },
                ]),
            ])
        );
    }

    #[test]
    fn extract_assertion_for_update_float() {
        let node_id = NodeId::new();
        let event = Event::PatchContent {
            node_id: node_id.clone(),
            patch: ContentPatch::UpdateReal(1.0),
        };
        assert_eq!(
            extract_assertion(&event),
            Assertion::All(vec![
                Assertion::NodeExists(&node_id),
                Assertion::ContentKind {
                    node_id: &node_id,
                    kind: ContentKind::Real,
                },
                Assertion::Any(vec![
                    Assertion::Owner,
                    Assertion::NodeAllows {
                        node_id: &node_id,
                        operation: NodeOperation::UpdateReal,
                    },
                ]),
            ])
        );
    }

    #[test]
    fn extract_assertion_for_update_rational() {
        let node_id = NodeId::new();
        let event = Event::PatchContent {
            node_id: node_id.clone(),
            patch: ContentPatch::UpdateRational(1, 2),
        };
        assert_eq!(
            extract_assertion(&event),
            Assertion::All(vec![
                Assertion::NodeExists(&node_id),
                Assertion::ContentKind {
                    node_id: &node_id,
                    kind: ContentKind::Rational,
                },
                Assertion::Any(vec![
                    Assertion::Owner,
                    Assertion::NodeAllows {
                        node_id: &node_id,
                        operation: NodeOperation::UpdateRational,
                    },
                ]),
            ])
        );
    }

    #[test]
    fn extract_assertion_for_update_apply_type() {
        let node_id = NodeId::new();
        let event = Event::PatchContent {
            node_id: node_id.clone(),
            patch: ContentPatch::UpdateApply {
                ty: Type::Real,
                link_name: LinkName::None,
            },
        };
        assert_eq!(
            extract_assertion(&event),
            Assertion::All(vec![
                Assertion::NodeExists(&node_id),
                Assertion::ContentKind {
                    node_id: &node_id,
                    kind: ContentKind::Apply,
                },
                Assertion::Any(vec![
                    Assertion::Owner,
                    Assertion::NodeAllows {
                        node_id: &node_id,
                        operation: NodeOperation::UpdateApply,
                    },
                ]),
            ])
        );
    }

    #[test]
    fn extract_assertion_for_insert_operand() {
        let node_id = NodeId::new();
        let operand_id = NodeId::new();
        let event = Event::PatchOperand {
            node_id: node_id.clone(),
            patch: OperandPatch::Insert {
                index: 2,
                node_id: operand_id.clone(),
            },
        };
        assert_eq!(
            extract_assertion(&event),
            Assertion::All(vec![
                Assertion::NodeExists(&node_id),
                Assertion::NotReferenced(&operand_id),
                Assertion::NoOperandLoop {
                    node_id: &node_id,
                    operand_id: &operand_id,
                },
                Assertion::OperandsHasSize {
                    node_id: &node_id,
                    size: 2,
                },
                Assertion::Any(vec![
                    Assertion::Owner,
                    Assertion::NodeAllows {
                        node_id: &node_id,
                        operation: NodeOperation::InsertOperand,
                    },
                ]),
            ])
        );
    }

    #[test]
    fn extract_assertion_for_remove_operand() {
        let node_id = NodeId::new();
        let event = Event::PatchOperand {
            node_id: node_id.clone(),
            patch: OperandPatch::Remove { index: 2 },
        };
        assert_eq!(
            extract_assertion(&event),
            Assertion::All(vec![
                Assertion::NodeExists(&node_id),
                Assertion::OperandsHasSize {
                    node_id: &node_id,
                    size: 3,
                },
                Assertion::Any(vec![
                    Assertion::Owner,
                    Assertion::NodeAllows {
                        node_id: &node_id,
                        operation: NodeOperation::RemoveOperand,
                    },
                ]),
            ])
        );
    }

    #[test]
    fn extract_assertion_for_move_operand_backward() {
        let node_id = NodeId::new();
        let event = Event::PatchOperand {
            node_id: node_id.clone(),
            patch: OperandPatch::Move { from: 4, to: 3 },
        };
        assert_eq!(
            extract_assertion(&event),
            Assertion::All(vec![
                Assertion::NodeExists(&node_id),
                Assertion::OperandsHasSize {
                    node_id: &node_id,
                    size: 5,
                },
                Assertion::Any(vec![
                    Assertion::Owner,
                    Assertion::NodeAllows {
                        node_id: &node_id,
                        operation: NodeOperation::MoveOperand,
                    },
                ]),
            ])
        );
    }

    #[test]
    fn extract_assertion_for_move_operand_forward() {
        let node_id = NodeId::new();
        let event = Event::PatchOperand {
            node_id: node_id.clone(),
            patch: OperandPatch::Move { from: 2, to: 3 },
        };
        assert_eq!(
            extract_assertion(&event),
            Assertion::All(vec![
                Assertion::NodeExists(&node_id),
                Assertion::OperandsHasSize {
                    node_id: &node_id,
                    size: 4,
                },
                Assertion::Any(vec![
                    Assertion::Owner,
                    Assertion::NodeAllows {
                        node_id: &node_id,
                        operation: NodeOperation::MoveOperand,
                    },
                ]),
            ])
        );
    }

    #[test]
    fn extract_assertion_for_update_attribute() {
        let node_id = NodeId::new();
        let event = Event::PatchAttribute {
            node_id: node_id.clone(),
            patch: AttributePatch::Update {
                key: Type::Real,
                value: 0.into(),
            },
        };
        assert_eq!(
            extract_assertion(&event),
            Assertion::All(vec![
                Assertion::NodeExists(&node_id),
                Assertion::Any(vec![
                    Assertion::Owner,
                    Assertion::NodeAllows {
                        node_id: &node_id,
                        operation: NodeOperation::UpdateAttribute(Type::Real),
                    },
                ]),
            ])
        );
    }

    #[test]
    fn extract_assertion_for_remove_attribute() {
        let node_id = NodeId::new();
        let event = Event::PatchAttribute {
            node_id: node_id.clone(),
            patch: AttributePatch::Remove { key: Type::Real },
        };
        assert_eq!(
            extract_assertion(&event),
            Assertion::All(vec![
                Assertion::NodeExists(&node_id),
                Assertion::Any(vec![
                    Assertion::Owner,
                    Assertion::NodeAllows {
                        node_id: &node_id,
                        operation: NodeOperation::RemoveAttribute(Type::Real),
                    },
                ]),
            ])
        );
    }

    #[test]
    fn extract_assertion_for_add_snapshot() {
        let event = Event::AddSnapshot {
            index: 0,
            snapshot: Default::default(),
        };
        assert_eq!(
            extract_assertion(&event),
            Assertion::Any(vec![
                Assertion::Owner,
                Assertion::SpaceAllows(SpaceOperation::AddSnapshot)
            ]),
        );
    }

    #[test]
    fn extract_assertion_for_update_space_rule() {
        let event = Event::UpdateSpaceRules {
            rules: Rules::default(),
        };
        assert_eq!(extract_assertion(&event), Assertion::Owner,);
    }

    #[test]
    fn extract_assertion_for_update_node_rule() {
        let node_id = NodeId::new();
        let event = Event::UpdateNodeRules {
            node_id: node_id.clone(),
            rules: Rules::default(),
        };
        assert_eq!(
            extract_assertion(&event),
            Assertion::All(vec![
                Assertion::NodeExists(&node_id),
                Assertion::Any(vec![
                    Assertion::Owner,
                    Assertion::NodeAllows {
                        node_id: &node_id,
                        operation: NodeOperation::UpdateRules,
                    },
                ]),
            ])
        );
    }

    #[test]
    fn extract_assertion_for_update_operand_rule() {
        let node_id = NodeId::new();
        let event = Event::UpdateOperandRules {
            node_id: node_id.clone(),
            rules: Rules::default(),
        };
        assert_eq!(
            extract_assertion(&event),
            Assertion::All(vec![
                Assertion::NodeExists(&node_id),
                Assertion::Any(vec![
                    Assertion::Owner,
                    Assertion::NodeAllows {
                        node_id: &node_id,
                        operation: NodeOperation::UpdateOperandRules,
                    },
                ]),
            ])
        );
    }
}
