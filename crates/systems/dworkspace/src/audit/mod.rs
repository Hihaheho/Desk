pub(self) mod assertion;
pub mod execute_assertion;
pub(super) mod extract_assertion;

use components::event::EventEntry;

use crate::Kernel;

use self::execute_assertion::AssertionError;

impl Kernel {
    pub fn audit(&self, entry: &EventEntry) -> Result<(), AssertionError> {
        let assertion = extract_assertion::extract_assertion(&entry.event);
        self.execute_assertion(&entry.user_id, assertion)
    }
}

#[cfg(test)]
mod tests {

    use components::{
        content::Content, event::Event, patch::OperandPatch, rules::Rules, user::UserId,
    };
    use deskc_ids::NodeId;

    use crate::repository::TestRepository;

    use super::*;

    #[test]
    fn initial_add_owner_is_always_allowed() {
        let kernel = Kernel::new(TestRepository::default());

        assert!(kernel
            .audit(&EventEntry {
                index: 0,
                user_id: UserId("a".into()),
                event: Event::AddOwner {
                    user_id: UserId("a".into()),
                }
            })
            .is_ok());
    }

    #[test]
    fn any_event_allowed_for_owners() {
        let mut kernel = Kernel::new(TestRepository::default());
        kernel.snapshot.handle_event(&Event::AddOwner {
            user_id: UserId("a".into()),
        });
        assert!(kernel
            .audit(&EventEntry {
                index: 0,
                user_id: UserId("a".into()),
                event: Event::AddOwner {
                    user_id: UserId("b".into()),
                },
            })
            .is_ok());
        assert!(kernel
            .audit(&EventEntry {
                index: 0,
                user_id: UserId("a".into()),
                event: Event::UpdateSpaceRules {
                    rules: Rules::default()
                }
            })
            .is_ok());
    }

    #[test]
    fn update_space_rule_denied() {
        let mut kernel = Kernel::new(TestRepository::default());
        kernel.snapshot.handle_event(&Event::AddOwner {
            user_id: UserId("a".into()),
        });
        assert!(kernel
            .audit(&EventEntry {
                index: 0,
                user_id: UserId("b".into()),
                event: Event::UpdateSpaceRules {
                    rules: Rules::default()
                }
            })
            .is_err());
    }

    #[test]
    fn prevent_loop() {
        let node_a = NodeId::new();
        let node_b = NodeId::new();
        let mut kernel = Kernel::new(TestRepository::default());
        kernel.snapshot.owners.insert(UserId("a".into()));

        kernel.handle_event(&Event::CreateNode {
            node_id: node_a.clone(),
            content: Content::Integer(0),
        });
        kernel.handle_event(&Event::CreateNode {
            node_id: node_b.clone(),
            content: Content::Integer(0),
        });
        kernel.handle_event(&Event::PatchOperand {
            node_id: node_a.clone(),
            patch: OperandPatch::Insert {
                index: 0,
                node_id: node_b.clone(),
            },
        });
        assert!(kernel
            .audit(&EventEntry {
                index: 0,
                user_id: UserId("a".into()),
                event: Event::PatchOperand {
                    node_id: node_a.clone(),
                    patch: OperandPatch::Insert {
                        index: 0,
                        node_id: node_b.clone(),
                    },
                },
            })
            .is_ok());
        assert!(kernel
            .audit(&EventEntry {
                index: 0,
                user_id: UserId("a".into()),
                event: Event::PatchOperand {
                    node_id: node_b,
                    patch: OperandPatch::Insert {
                        index: 0,
                        node_id: node_a,
                    },
                },
            })
            .is_err());
    }

    #[test]
    fn index_out_of_range() {
        let node_a = NodeId::new();
        let node_b = NodeId::new();
        let node_c = NodeId::new();
        let node_d = NodeId::new();
        let mut kernel = Kernel::new(TestRepository::default());
        kernel.snapshot.owners.insert(UserId("a".into()));
        kernel.handle_event(&Event::CreateNode {
            node_id: node_a.clone(),
            content: Content::Integer(0),
        });
        kernel.handle_event(&Event::CreateNode {
            node_id: node_b.clone(),
            content: Content::Integer(0),
        });
        kernel.handle_event(&Event::CreateNode {
            node_id: node_c.clone(),
            content: Content::Integer(0),
        });
        kernel.handle_event(&Event::CreateNode {
            node_id: node_d.clone(),
            content: Content::Integer(0),
        });
        kernel.handle_event(&Event::PatchOperand {
            node_id: node_a.clone(),
            patch: OperandPatch::Insert {
                index: 0,
                node_id: node_b.clone(),
            },
        });
        kernel.handle_event(&Event::PatchOperand {
            node_id: node_a.clone(),
            patch: OperandPatch::Insert {
                index: 1,
                node_id: node_b.clone(),
            },
        });
        // insert at 2
        assert_eq!(
            kernel.audit(&EventEntry {
                index: 0,
                user_id: UserId("a".into()),
                event: Event::PatchOperand {
                    node_id: node_a.clone(),
                    patch: OperandPatch::Insert {
                        index: 2,
                        node_id: node_d.clone(),
                    },
                },
            }),
            Ok(())
        );
        // insert at 3 (out of range)
        assert_eq!(
            kernel.audit(&EventEntry {
                index: 0,
                user_id: UserId("a".into()),
                event: Event::PatchOperand {
                    node_id: node_a.clone(),
                    patch: OperandPatch::Insert {
                        index: 3,
                        node_id: node_d.clone(),
                    },
                },
            }),
            Err(AssertionError::InsufficientOperands {
                node_id: node_a.clone(),
                target: 3,
                actual: 2
            })
        );
        // remove at 1
        assert_eq!(
            kernel.audit(&EventEntry {
                index: 0,
                user_id: UserId("a".into()),
                event: Event::PatchOperand {
                    node_id: node_a.clone(),
                    patch: OperandPatch::Remove { index: 1 },
                },
            }),
            Ok(())
        );
        // remove at 2 (out of range)
        assert_eq!(
            kernel.audit(&EventEntry {
                index: 0,
                user_id: UserId("a".into()),
                event: Event::PatchOperand {
                    node_id: node_a.clone(),
                    patch: OperandPatch::Remove { index: 2 },
                },
            }),
            Err(AssertionError::InsufficientOperands {
                node_id: node_a.clone(),
                target: 3,
                actual: 2
            })
        );
        // move from 1 to 0
        assert_eq!(
            kernel.audit(&EventEntry {
                index: 0,
                user_id: UserId("a".into()),
                event: Event::PatchOperand {
                    node_id: node_a.clone(),
                    patch: OperandPatch::Move { from: 1, to: 0 },
                },
            }),
            Ok(())
        );
        // move from 0 to 1
        assert_eq!(
            kernel.audit(&EventEntry {
                index: 0,
                user_id: UserId("a".into()),
                event: Event::PatchOperand {
                    node_id: node_a.clone(),
                    patch: OperandPatch::Move { from: 0, to: 1 },
                },
            }),
            Ok(())
        );
        // move from 1 to 2 (out of range)
        assert_eq!(
            kernel.audit(&EventEntry {
                index: 0,
                user_id: UserId("a".into()),
                event: Event::PatchOperand {
                    node_id: node_a.clone(),
                    patch: OperandPatch::Move { from: 1, to: 2 },
                },
            }),
            Err(AssertionError::InsufficientOperands {
                node_id: node_a.clone(),
                target: 3,
                actual: 2
            })
        );
        // move from 2 to 1 (out of range)
        assert_eq!(
            kernel.audit(&EventEntry {
                index: 0,
                user_id: UserId("a".into()),
                event: Event::PatchOperand {
                    node_id: node_a.clone(),
                    patch: OperandPatch::Move { from: 2, to: 1 },
                },
            }),
            Err(AssertionError::InsufficientOperands {
                node_id: node_a.clone(),
                target: 3,
                actual: 2
            })
        );
    }
}
