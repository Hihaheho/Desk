mod audit;
mod event;
mod hirs;
mod history;
mod query_result;
mod repository;
mod snapshot;

use audit::audit;
use dkernel_card::rules::AuditResponse;
use hirs::Hirs;
use history::History;
use repository::Repository;
use snapshot::Snapshot;

pub struct Kernel {
    repository: Box<dyn Repository>,
    hirs: Hirs,
    pub snapshot: Snapshot,
    pub history: History,
}

impl Kernel {
    pub fn new(repository: impl Repository + 'static) -> Self {
        Self {
            repository: Box::new(repository),
            hirs: Default::default(),
            snapshot: Default::default(),
            history: Default::default(),
        }
    }

    pub fn process(&mut self) {
        let entries = self.repository.poll();
        for entry in entries {
            if audit(&self.snapshot, &entry) == AuditResponse::Allowed {
                self.hirs.handle_event(&entry.event);
                self.history.handle_event(&self.snapshot, &entry.event);
                self.snapshot.handle_event(&self.hirs, &entry.event);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::event::{Event, EventEntry};
    use deskc_hir::expr::Literal;
    use deskc_hir::ty::Type as HirType;
    use deskc_hir::{
        expr::Expr,
        meta::{Meta, WithMeta},
    };
    use deskc_ids::{CardId, FileId, IrId, LinkName, NodeId, UserId};
    use deskc_types::Type;
    use dkernel_card::patch::FilePatch;
    use dkernel_card::rules::{NodeOperation, Rules};
    use dkernel_card::{content::Content, flat_node::NodeRef, patch::ChildrenPatch};
    use hirs::HirQueries;
    use std::sync::Arc;
    use uuid::Uuid;

    use super::*;

    #[mry::mry]
    #[derive(Default)]
    pub struct TestRepository {}

    #[mry::mry]
    impl Repository for TestRepository {
        fn poll(&mut self) -> Vec<EventEntry> {
            todo!()
        }
        fn commit(&mut self, log: Event) {
            todo!()
        }
        fn add_owner(&mut self, user_id: UserId) {
            todo!()
        }
        fn remove_owner(&mut self, user_id: UserId) {
            todo!()
        }
    }

    #[test]
    fn integration() {
        let mut repository = TestRepository::default();

        let user_a = UserId("a".into());
        let user_b = UserId("b".into());
        let node_a = NodeId(Uuid::new_v4());
        let node_b = NodeId(Uuid::new_v4());
        let file_id = FileId(Uuid::new_v4());
        let card_id = CardId(Uuid::new_v4());

        repository.mock_poll().returns(vec![
            EventEntry {
                index: 0,
                user_id: user_a.clone(),
                event: Event::AddOwner {
                    user_id: user_a.clone(),
                },
            },
            EventEntry {
                index: 0,
                user_id: user_a.clone(),
                event: Event::AddFile(file_id.clone()),
            },
            EventEntry {
                index: 0,
                user_id: user_a.clone(),
                event: Event::PatchFile {
                    file_id: file_id.clone(),
                    patch: FilePatch::UpdateRules {
                        rules: Rules {
                            default: [NodeOperation::AddNode, NodeOperation::PatchChildrenInsert]
                                .into_iter()
                                .collect(),
                            users: Default::default(),
                        },
                    },
                },
            },
            EventEntry {
                index: 0,
                user_id: user_b.clone(),
                event: Event::AddOwner {
                    user_id: user_b.clone(),
                },
            },
            EventEntry {
                index: 0,
                user_id: user_a.clone(),
                event: Event::AddNode {
                    node_id: node_a.clone(),
                    file_id: file_id.clone(),
                    content: Content::Apply(Type::Function {
                        parameters: vec![Type::String],
                        body: Box::new(Type::Number),
                    }),
                },
            },
            EventEntry {
                index: 1,
                user_id: user_b.clone(),
                event: Event::AddNode {
                    node_id: node_b.clone(),
                    file_id: file_id.clone(),
                    content: Content::String("string".into()),
                },
            },
            EventEntry {
                index: 1,
                user_id: user_b.clone(),
                event: Event::PatchChildren {
                    node_id: node_a.clone(),
                    patch: ChildrenPatch::Insert {
                        index: 0,
                        node: NodeRef::Node(node_b.clone()),
                    },
                },
            },
            EventEntry {
                index: 1,
                user_id: user_a.clone(),
                event: Event::AddCard {
                    card_id: card_id.clone(),
                    node_id: node_a.clone(),
                },
            },
        ]);

        let mut kernel = Kernel::new(repository);
        kernel.process();

        assert_eq!(kernel.snapshot.flat_nodes.len(), 2);
        assert_eq!(kernel.snapshot.owners.len(), 1);
        assert_eq!(
            kernel.hirs.hir(card_id),
            Ok(Arc::new(WithMeta {
                id: IrId::default(),
                meta: Meta::default(),
                value: Expr::Apply {
                    function: WithMeta {
                        id: IrId::default(),
                        meta: Meta::default(),
                        value: HirType::Function {
                            parameter: Box::new(WithMeta {
                                id: IrId::default(),
                                meta: Meta::default(),
                                value: HirType::String
                            }),
                            body: Box::new(WithMeta {
                                id: IrId::default(),
                                meta: Meta::default(),
                                value: HirType::Number
                            }),
                        }
                    },
                    link_name: LinkName::None,
                    arguments: vec![WithMeta {
                        id: IrId::default(),
                        meta: Meta::default(),
                        value: Expr::Literal(Literal::String("string".into()))
                    }]
                }
            }))
        );
    }
}
