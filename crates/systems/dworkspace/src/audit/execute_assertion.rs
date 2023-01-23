use components::{
    content::ContentKind,
    rules::{NodeOperation, SpaceOperation},
    user::UserId,
};
use deskc_ids::NodeId;

use crate::{references::ReferencesQueries, Workspace};

use super::assertion::Assertion;

#[derive(Debug, PartialEq)]
pub enum AssertionError {
    SpaceDenied(SpaceOperation),
    NodeDenied {
        node_id: NodeId,
        operation: NodeOperation,
    },
    ParentDenied {
        node_id: NodeId,
        operation: NodeOperation,
    },
    NotOwner,
    NodeNotFound(NodeId),
    OperandNotFound {
        node_id: NodeId,
        operand_id: NodeId,
    },
    Referenced(NodeId),
    OperandLoop {
        node_id: NodeId,
        operand_id: NodeId,
    },
    ContentKindMismatch {
        node_id: NodeId,
        expected: ContentKind,
        actual: ContentKind,
    },
    // All(Vec<AssertionError>) must not be here, because it causes file system's crash
    Any(Vec<AssertionError>),
    InsufficientOperands {
        node_id: NodeId,
        target: usize,
        actual: usize,
    },
    MovingItself {
        node_id: NodeId,
    },
}

impl Workspace {
    pub fn execute_assertion(
        &self,
        user_id: &UserId,
        assertion: Assertion,
    ) -> Result<(), AssertionError> {
        match assertion {
            Assertion::SpaceAllows(operation) => {
                if self.snapshot.rules.user_has_operation(user_id, &operation) {
                    Ok(())
                } else {
                    Err(AssertionError::SpaceDenied(operation))
                }
            }
            Assertion::NodeAllows { node_id, operation } => {
                if !self
                    .snapshot
                    .flat_nodes
                    .get(&node_id)
                    .unwrap()
                    .rules
                    .user_has_operation(user_id, &operation)
                {
                    return Err(AssertionError::NodeDenied {
                        node_id: node_id.clone(),
                        operation,
                    });
                }
                if let Some(rules) = self.references.lock().parent_rules(node_id.clone()) {
                    if !rules.user_has_operation(user_id, &operation) {
                        return Err(AssertionError::ParentDenied {
                            node_id: node_id.clone(),
                            operation,
                        });
                    }
                }
                Ok(())
            }
            Assertion::Owner => {
                if self.snapshot.owners.contains(user_id) {
                    Ok(())
                } else {
                    Err(AssertionError::NotOwner)
                }
            }
            Assertion::NoOwner => {
                if self.snapshot.owners.is_empty() {
                    Ok(())
                } else {
                    Err(AssertionError::NotOwner)
                }
            }
            Assertion::NodeExists(node_id) => {
                if self.snapshot.flat_nodes.contains_key(&node_id) {
                    Ok(())
                } else {
                    Err(AssertionError::NodeNotFound(node_id.clone()))
                }
            }
            Assertion::HasOperand {
                node_id,
                operand_id,
            } => {
                if self
                    .snapshot
                    .flat_nodes
                    .get(&node_id)
                    .unwrap()
                    .operands
                    .contains(&operand_id)
                {
                    Ok(())
                } else {
                    Err(AssertionError::OperandNotFound {
                        node_id: node_id.clone(),
                        operand_id: operand_id.clone(),
                    })
                }
            }
            Assertion::NotReferenced(node_id) => {
                if self
                    .references
                    .lock()
                    .references(node_id.clone())
                    .is_empty()
                {
                    Ok(())
                } else {
                    Err(AssertionError::Referenced(node_id.clone()))
                }
            }
            Assertion::NoOperandLoop {
                node_id,
                operand_id,
            } => {
                if self
                    .loop_detector
                    .does_make_loop_insert_operand(node_id, operand_id)
                {
                    Err(AssertionError::OperandLoop {
                        node_id: node_id.clone(),
                        operand_id: operand_id.clone(),
                    })
                } else {
                    Ok(())
                }
            }
            Assertion::OperandsHasSize { node_id, size } => {
                let actual = self
                    .snapshot
                    .flat_nodes
                    .get(&node_id)
                    .unwrap()
                    .operands
                    .len();
                if actual >= size {
                    Ok(())
                } else {
                    Err(AssertionError::InsufficientOperands {
                        node_id: node_id.clone(),
                        target: size,
                        actual,
                    })
                }
            }
            Assertion::ContentKind {
                node_id,
                kind: expected,
            } => {
                let actual = self
                    .snapshot
                    .flat_nodes
                    .get(&node_id)
                    .unwrap()
                    .content
                    .kind();
                if actual == expected {
                    Ok(())
                } else {
                    Err(AssertionError::ContentKindMismatch {
                        node_id: node_id.clone(),
                        expected,
                        actual,
                    })
                }
            }
            Assertion::All(assertions) => {
                let result: Result<Vec<_>, _> = assertions
                    .into_iter()
                    .map(|assertion| self.execute_assertion(user_id, assertion))
                    .collect();
                match result {
                    Ok(_) => Ok(()),
                    Err(err) => Err(err),
                }
            }
            Assertion::Any(assertions) => {
                let assertions_len = assertions.len();
                let errs: Vec<_> = assertions
                    .into_iter()
                    .map(|assertion| self.execute_assertion(user_id, assertion))
                    .filter_map(|result| match result {
                        Ok(_) => None,
                        Err(err) => Some(err),
                    })
                    .collect();
                if errs.len() == assertions_len {
                    Err(AssertionError::Any(errs))
                } else {
                    Ok(())
                }
            }
            Assertion::Contradiction(reason) => Err(reason),
        }
    }
}

#[cfg(test)]
mod tests {
    use components::{
        content::Content,
        event::Event,
        flat_node::FlatNode,
        patch::{OperandPatch, OperandPosition},
        rules::{Rules, SpaceOperation},
        user::UserId,
    };

    use crate::repository::TestRepository;

    use super::*;

    #[test]
    fn space_denies() {
        let kernel = Workspace::new(TestRepository::default());
        let assertion = Assertion::SpaceAllows(SpaceOperation::CreateNode);
        assert_eq!(
            kernel.execute_assertion(&UserId("a".into()), assertion),
            Err(AssertionError::SpaceDenied(SpaceOperation::CreateNode))
        );
    }

    #[test]
    fn space_allows() {
        let mut kernel = Workspace::new(TestRepository::default());
        kernel
            .snapshot
            .rules
            .default
            .insert(SpaceOperation::CreateNode);
        let assertion = Assertion::SpaceAllows(SpaceOperation::CreateNode);
        assert_eq!(
            kernel.execute_assertion(&UserId("a".into()), assertion),
            Ok(())
        );
    }

    #[test]
    fn node_denies() {
        let mut kernel = Workspace::new(TestRepository::default());
        let node_id = NodeId::new();
        kernel
            .snapshot
            .flat_nodes
            .insert(node_id.clone(), FlatNode::new(Content::Integer(0)));
        let assertion = Assertion::NodeAllows {
            node_id,
            operation: NodeOperation::RemoveNode,
        };
        assert_eq!(
            kernel.execute_assertion(&UserId("a".into()), assertion),
            Err(AssertionError::NodeDenied {
                node_id,
                operation: NodeOperation::RemoveNode
            })
        );
    }

    #[test]
    fn node_allows() {
        let mut kernel = Workspace::new(TestRepository::default());
        let node_id = NodeId::new();
        kernel.handle_event(&Event::CreateNode {
            node_id: node_id.clone(),
            content: Content::Integer(0),
        });
        kernel.handle_event(&Event::UpdateNodeRules {
            node_id: node_id.clone(),
            rules: Rules {
                default: [NodeOperation::RemoveNode].iter().cloned().collect(),
                ..Default::default()
            },
        });
        let assertion = Assertion::NodeAllows {
            node_id,
            operation: NodeOperation::RemoveNode,
        };
        assert_eq!(
            kernel.execute_assertion(&UserId("a".into()), assertion),
            Ok(())
        );
    }

    #[test]
    fn parent_denies() {
        let mut kernel = Workspace::new(TestRepository::default());
        let node_id = NodeId::new();
        let parent_id = NodeId::new();
        kernel.handle_event(&Event::CreateNode {
            node_id: node_id.clone(),
            content: Content::Integer(0),
        });
        kernel.handle_event(&Event::CreateNode {
            node_id: parent_id.clone(),
            content: Content::Integer(0),
        });
        kernel.handle_event(&Event::PatchOperand {
            node_id: parent_id,
            patch: OperandPatch::Insert {
                position: OperandPosition::First,
                node_id: node_id.clone(),
            },
        });
        kernel.handle_event(&Event::UpdateNodeRules {
            node_id: node_id.clone(),
            rules: Rules {
                default: [NodeOperation::RemoveNode].iter().cloned().collect(),
                ..Default::default()
            },
        });
        let assertion = Assertion::NodeAllows {
            node_id,
            operation: NodeOperation::RemoveNode,
        };
        assert_eq!(
            kernel.execute_assertion(&UserId("a".into()), assertion),
            Err(AssertionError::ParentDenied {
                node_id,
                operation: NodeOperation::RemoveNode
            })
        );
    }

    #[test]
    fn parent_allows() {
        let mut kernel = Workspace::new(TestRepository::default());
        let node_id = NodeId::new();
        let parent_id = NodeId::new();
        kernel.handle_event(&Event::CreateNode {
            node_id: node_id.clone(),
            content: Content::Integer(0),
        });
        kernel.handle_event(&Event::CreateNode {
            node_id: parent_id.clone(),
            content: Content::Integer(0),
        });
        kernel.handle_event(&Event::PatchOperand {
            node_id: parent_id.clone(),
            patch: OperandPatch::Insert {
                position: OperandPosition::First,
                node_id: node_id.clone(),
            },
        });
        kernel.handle_event(&Event::UpdateNodeRules {
            node_id: node_id.clone(),
            rules: Rules {
                default: [NodeOperation::RemoveNode].iter().cloned().collect(),
                ..Default::default()
            },
        });
        kernel.handle_event(&Event::UpdateOperandRules {
            node_id: parent_id.clone(),
            rules: Rules {
                default: [NodeOperation::RemoveNode].iter().cloned().collect(),
                ..Default::default()
            },
        });
        kernel
            .snapshot
            .flat_nodes
            .get_mut(&parent_id)
            .unwrap()
            .rules
            .default
            .insert(NodeOperation::RemoveNode);
        let assertion = Assertion::NodeAllows {
            node_id,
            operation: NodeOperation::RemoveNode,
        };
        assert_eq!(
            kernel.execute_assertion(&UserId("a".into()), assertion),
            Ok(())
        );
    }

    #[test]
    fn owner_denies() {
        let kernel = Workspace::new(TestRepository::default());
        let assertion = Assertion::Owner;
        assert_eq!(
            kernel.execute_assertion(&UserId("b".into()), assertion),
            Err(AssertionError::NotOwner)
        );
    }

    #[test]
    fn owner_allows() {
        let mut kernel = Workspace::new(TestRepository::default());
        kernel.handle_event(&Event::AddOwner {
            user_id: UserId("a".into()),
        });
        let assertion = Assertion::Owner;
        assert_eq!(
            kernel.execute_assertion(&UserId("a".into()), assertion),
            Ok(())
        );
    }

    #[test]
    fn no_owner_denies() {
        let mut kernel = Workspace::new(TestRepository::default());
        kernel.handle_event(&Event::AddOwner {
            user_id: UserId("a".into()),
        });
        let assertion = Assertion::NoOwner;
        assert_eq!(
            kernel.execute_assertion(&UserId("a".into()), assertion),
            Err(AssertionError::NotOwner)
        );
    }

    #[test]
    fn no_owner_allows() {
        let kernel = Workspace::new(TestRepository::default());
        let assertion = Assertion::NoOwner;
        assert_eq!(
            kernel.execute_assertion(&UserId("a".into()), assertion),
            Ok(())
        );
    }

    #[test]
    fn node_exists_denies() {
        let kernel = Workspace::new(TestRepository::default());
        let node_id = NodeId::new();
        let assertion = Assertion::NodeExists(node_id);
        assert_eq!(
            kernel.execute_assertion(&UserId("a".into()), assertion),
            Err(AssertionError::NodeNotFound(node_id))
        );
    }

    #[test]
    fn node_exists_allows() {
        let mut kernel = Workspace::new(TestRepository::default());
        let node_id = NodeId::new();
        kernel.handle_event(&Event::CreateNode {
            node_id: node_id.clone(),
            content: Content::Integer(0),
        });
        let assertion = Assertion::NodeExists(node_id);
        assert_eq!(
            kernel.execute_assertion(&UserId("a".into()), assertion),
            Ok(())
        );
    }

    #[test]
    fn has_operand() {
        let mut kernel = Workspace::new(TestRepository::default());
        let node_a = NodeId::new();
        let node_b = NodeId::new();
        let node_c = NodeId::new();
        kernel.handle_event(&Event::CreateNode {
            node_id: node_a.clone(),
            content: Content::Integer(0),
        });
        kernel.handle_event(&Event::CreateNode {
            node_id: node_b,
            content: Content::Integer(0),
        });
        kernel.handle_event(&Event::CreateNode {
            node_id: node_c,
            content: Content::Integer(0),
        });
        kernel.handle_event(&Event::PatchOperand {
            node_id: node_b,
            patch: OperandPatch::Insert {
                position: OperandPosition::First,
                node_id: node_c,
            },
        });
        let assertion = Assertion::HasOperand {
            node_id: node_a,
            operand_id: node_c,
        };
        assert_eq!(
            kernel.execute_assertion(&UserId("a".into()), assertion),
            Err(AssertionError::OperandNotFound {
                node_id: node_a,
                operand_id: node_c
            })
        );
        let assertion = Assertion::HasOperand {
            node_id: node_b,
            operand_id: node_c,
        };
        assert_eq!(
            kernel.execute_assertion(&UserId("a".into()), assertion),
            Ok(())
        );
    }

    #[test]
    fn not_referenced_denies() {
        let mut kernel = Workspace::new(TestRepository::default());
        let node_id = NodeId::new();
        let parent_id = NodeId::new();
        kernel.handle_event(&Event::CreateNode {
            node_id: node_id.clone(),
            content: Content::Integer(0),
        });
        kernel.handle_event(&Event::CreateNode {
            node_id: parent_id.clone(),
            content: Content::Integer(0),
        });
        kernel.handle_event(&Event::PatchOperand {
            node_id: parent_id,
            patch: OperandPatch::Insert {
                position: OperandPosition::First,
                node_id: node_id.clone(),
            },
        });
        let assertion = Assertion::NotReferenced(node_id);
        assert_eq!(
            kernel.execute_assertion(&UserId("a".into()), assertion),
            Err(AssertionError::Referenced(node_id))
        );
    }

    #[test]
    fn not_referenced_allows() {
        let mut kernel = Workspace::new(TestRepository::default());
        let node_id = NodeId::new();
        let parent_id = NodeId::new();
        kernel.handle_event(&Event::CreateNode {
            node_id: node_id.clone(),
            content: Content::Integer(0),
        });
        kernel.handle_event(&Event::CreateNode {
            node_id: parent_id,
            content: Content::Integer(0),
        });
        let assertion = Assertion::NotReferenced(node_id);
        assert_eq!(
            kernel.execute_assertion(&UserId("a".into()), assertion),
            Ok(())
        );
    }

    #[test]
    fn no_operand_loop_denies() {
        let mut kernel = Workspace::new(TestRepository::default());
        let node_id = NodeId::new();
        let parent_id = NodeId::new();
        kernel.handle_event(&Event::CreateNode {
            node_id: node_id.clone(),
            content: Content::Integer(0),
        });
        kernel.handle_event(&Event::CreateNode {
            node_id: parent_id.clone(),
            content: Content::Integer(0),
        });
        kernel.handle_event(&Event::PatchOperand {
            node_id: parent_id.clone(),
            patch: OperandPatch::Insert {
                position: OperandPosition::First,
                node_id: node_id.clone(),
            },
        });
        let assertion = Assertion::NoOperandLoop {
            node_id,
            operand_id: parent_id,
        };
        assert_eq!(
            kernel.execute_assertion(&UserId("a".into()), assertion),
            Err(AssertionError::OperandLoop {
                node_id,
                operand_id: parent_id
            })
        );
    }

    #[test]
    fn no_operand_loop_allows() {
        let mut kernel = Workspace::new(TestRepository::default());
        let node_id = NodeId::new();
        let parent_id = NodeId::new();
        kernel.handle_event(&Event::CreateNode {
            node_id: node_id.clone(),
            content: Content::Integer(0),
        });
        kernel.handle_event(&Event::CreateNode {
            node_id: parent_id.clone(),
            content: Content::Integer(0),
        });
        let assertion = Assertion::NoOperandLoop {
            node_id,
            operand_id: parent_id,
        };
        assert_eq!(
            kernel.execute_assertion(&UserId("a".into()), assertion),
            Ok(())
        );
    }

    #[test]
    fn operands_has_size() {
        let mut kernel = Workspace::new(TestRepository::default());
        let node_id = NodeId::new();
        let node_a = NodeId::new();
        let node_b = NodeId::new();
        kernel.handle_event(&Event::CreateNode {
            node_id: node_id.clone(),
            content: Content::Integer(0),
        });
        kernel.handle_event(&Event::CreateNode {
            node_id: node_a.clone(),
            content: Content::Integer(0),
        });
        kernel.handle_event(&Event::CreateNode {
            node_id: node_b.clone(),
            content: Content::Integer(0),
        });
        kernel.handle_event(&Event::PatchOperand {
            node_id: node_id.clone(),
            patch: OperandPatch::Insert {
                position: OperandPosition::First,
                node_id: node_a,
            },
        });
        kernel.handle_event(&Event::PatchOperand {
            node_id: node_id.clone(),
            patch: OperandPatch::Insert {
                position: OperandPosition::Last,
                node_id: node_b,
            },
        });
        assert_eq!(
            kernel.execute_assertion(
                &UserId("a".into()),
                Assertion::OperandsHasSize { node_id, size: 0 }
            ),
            Ok(())
        );
        assert_eq!(
            kernel.execute_assertion(
                &UserId("a".into()),
                Assertion::OperandsHasSize { node_id, size: 1 }
            ),
            Ok(())
        );
        assert_eq!(
            kernel.execute_assertion(
                &UserId("a".into()),
                Assertion::OperandsHasSize { node_id, size: 2 }
            ),
            Ok(())
        );
        assert_eq!(
            kernel.execute_assertion(
                &UserId("a".into()),
                Assertion::OperandsHasSize { node_id, size: 3 }
            ),
            Err(AssertionError::InsufficientOperands {
                node_id: node_id.clone(),
                target: 3,
                actual: 2
            })
        );
        assert_eq!(
            kernel.execute_assertion(
                &UserId("a".into()),
                Assertion::OperandsHasSize { node_id, size: 4 }
            ),
            Err(AssertionError::InsufficientOperands {
                node_id: node_id.clone(),
                target: 4,
                actual: 2
            })
        );
    }

    #[test]
    fn content_kind_allows() {
        let mut kernel = Workspace::new(TestRepository::default());
        let node_id = NodeId::new();
        kernel.handle_event(&Event::CreateNode {
            node_id: node_id.clone(),
            content: Content::Integer(0),
        });
        assert_eq!(
            kernel.execute_assertion(
                &UserId("a".into()),
                Assertion::ContentKind {
                    node_id,
                    kind: ContentKind::Integer,
                }
            ),
            Ok(())
        );
    }

    #[test]
    fn content_kind_denies() {
        let mut kernel = Workspace::new(TestRepository::default());
        let node_id = NodeId::new();
        kernel.handle_event(&Event::CreateNode {
            node_id: node_id.clone(),
            content: Content::Integer(0),
        });
        assert_eq!(
            kernel.execute_assertion(
                &UserId("a".into()),
                Assertion::ContentKind {
                    node_id,
                    kind: ContentKind::String,
                }
            ),
            Err(AssertionError::ContentKindMismatch {
                node_id: node_id.clone(),
                expected: ContentKind::String,
                actual: ContentKind::Integer
            })
        );
    }

    #[test]
    fn all_allows() {
        let mut kernel = Workspace::new(TestRepository::default());
        let node_id = NodeId::new();
        kernel.handle_event(&Event::CreateNode {
            node_id: node_id.clone(),
            content: Content::Integer(0),
        });
        kernel.handle_event(&Event::UpdateNodeRules {
            node_id: node_id.clone(),
            rules: Rules {
                default: [NodeOperation::RemoveNode, NodeOperation::ReplaceContent]
                    .iter()
                    .cloned()
                    .collect(),
                ..Default::default()
            },
        });
        let assertion = Assertion::All(vec![
            Assertion::NodeAllows {
                node_id,
                operation: NodeOperation::RemoveNode,
            },
            Assertion::NodeAllows {
                node_id,
                operation: NodeOperation::ReplaceContent,
            },
        ]);
        assert_eq!(
            kernel.execute_assertion(&UserId("a".into()), assertion),
            Ok(())
        );
    }

    #[test]
    fn all_denies() {
        let mut kernel = Workspace::new(TestRepository::default());
        let node_id = NodeId::new();
        kernel.handle_event(&Event::CreateNode {
            node_id: node_id.clone(),
            content: Content::Integer(0),
        });
        kernel.handle_event(&Event::UpdateNodeRules {
            node_id: node_id.clone(),
            rules: Rules {
                default: [NodeOperation::RemoveNode].iter().cloned().collect(),
                ..Default::default()
            },
        });
        let assertion = Assertion::All(vec![
            Assertion::NodeAllows {
                node_id,
                operation: NodeOperation::RemoveNode,
            },
            Assertion::NodeAllows {
                node_id,
                operation: NodeOperation::ReplaceContent,
            },
            Assertion::NodeAllows {
                node_id,
                operation: NodeOperation::MoveOperand,
            },
        ]);
        assert_eq!(
            kernel.execute_assertion(&UserId("a".into()), assertion),
            Err(AssertionError::NodeDenied {
                node_id,
                operation: NodeOperation::ReplaceContent,
            })
        );
    }

    #[test]
    fn all_allows_empty() {
        let kernel = Workspace::new(TestRepository::default());
        let assertion = Assertion::All(vec![]);
        assert_eq!(
            kernel.execute_assertion(&UserId("a".into()), assertion),
            Ok(())
        );
    }

    #[test]
    fn any_allows() {
        let mut kernel = Workspace::new(TestRepository::default());
        let node_id = NodeId::new();
        kernel.handle_event(&Event::CreateNode {
            node_id: node_id.clone(),
            content: Content::Integer(0),
        });
        kernel.handle_event(&Event::UpdateNodeRules {
            node_id: node_id.clone(),
            rules: Rules {
                default: [NodeOperation::RemoveNode].iter().cloned().collect(),
                ..Default::default()
            },
        });
        let assertion = Assertion::Any(vec![
            Assertion::NodeAllows {
                node_id,
                operation: NodeOperation::RemoveNode,
            },
            Assertion::NodeAllows {
                node_id,
                operation: NodeOperation::ReplaceContent,
            },
        ]);
        assert_eq!(
            kernel.execute_assertion(&UserId("a".into()), assertion),
            Ok(())
        );
    }

    #[test]
    fn any_denies() {
        let mut kernel = Workspace::new(TestRepository::default());
        let node_id = NodeId::new();
        kernel.handle_event(&Event::CreateNode {
            node_id: node_id.clone(),
            content: Content::Integer(0),
        });
        kernel.handle_event(&Event::UpdateNodeRules {
            node_id: node_id.clone(),
            rules: Rules {
                default: [NodeOperation::RemoveNode].iter().cloned().collect(),
                ..Default::default()
            },
        });
        let assertion = Assertion::Any(vec![
            Assertion::NodeAllows {
                node_id,
                operation: NodeOperation::PatchString,
            },
            Assertion::NodeAllows {
                node_id,
                operation: NodeOperation::ReplaceContent,
            },
        ]);
        assert_eq!(
            kernel.execute_assertion(&UserId("a".into()), assertion),
            Err(AssertionError::Any(vec![
                AssertionError::NodeDenied {
                    node_id: node_id.clone(),
                    operation: NodeOperation::PatchString,
                },
                AssertionError::NodeDenied {
                    node_id,
                    operation: NodeOperation::ReplaceContent,
                },
            ]))
        );
    }

    #[test]
    fn any_denies_empty() {
        let kernel = Workspace::new(TestRepository::default());
        let assertion = Assertion::Any(vec![]);
        assert_eq!(
            kernel.execute_assertion(&UserId("a".into()), assertion),
            Err(AssertionError::Any(vec![]))
        );
    }

    #[test]
    fn tautology() {
        let kernel = Workspace::new(TestRepository::default());
        let assertion = Assertion::tautology();
        assert_eq!(
            kernel.execute_assertion(&UserId("a".into()), assertion),
            Ok(())
        );
    }

    #[test]
    fn contradiction() {
        let node_id = NodeId::new();
        let kernel = Workspace::new(TestRepository::default());
        let assertion = Assertion::Contradiction(AssertionError::MovingItself { node_id });
        assert_eq!(
            kernel.execute_assertion(&UserId("a".into()), assertion),
            Err(AssertionError::MovingItself { node_id })
        );
    }
}
