mod database;
mod event;
mod history;
mod query_result;
mod references;
mod repository;
mod snapshot;

use database::KernelStorage;
use database::Queries;
use history::History;
use references::References;
use repository::Repository;
use snapshot::Snapshot;

pub struct Kernel {
    repository: Box<dyn Repository>,
    db: KernelDatabase,
    pub snapshot: Snapshot,
    pub references: References,
    pub history: History,
}

#[salsa::database(KernelStorage)]
#[derive(Default)]
pub struct KernelDatabase {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for KernelDatabase {}

impl Kernel {
    pub fn new(repository: impl Repository + 'static) -> Self {
        Self {
            repository: Box::new(repository),
            db: Default::default(),
            snapshot: Default::default(),
            references: Default::default(),
            history: Default::default(),
        }
    }

    pub fn process(&mut self) {
        let logs = self.repository.poll();
        for log in logs {}
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use deskc_hir::expr::Literal;
    use deskc_hir::ty::Type as HirType;
    use deskc_hir::{
        expr::Expr,
        meta::{Meta, WithMeta},
    };
    use deskc_ids::{CardId, FileId, LinkName, UserId};
    use deskc_types::Type;
    use dkernel_card::{content::Content, flat_node::NodeRef, node::NodeId, patch::ChildrenPatch};
    use uuid::Uuid;

    use crate::event::{Event, EventEntry};

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
                log: Event::AddOwner {
                    user_id: user_a.clone(),
                },
            },
            EventEntry {
                index: 0,
                user_id: user_b.clone(),
                log: Event::AddOwner {
                    user_id: user_b.clone(),
                },
            },
            EventEntry {
                index: 0,
                user_id: user_a.clone(),
                log: Event::AddFile(file_id.clone()),
            },
            EventEntry {
                index: 0,
                user_id: user_a.clone(),
                log: Event::AddNode {
                    node_id: node_a.clone(),
                    content: Content::Apply(Type::Function {
                        parameters: vec![Type::String],
                        body: Box::new(Type::Number),
                    }),
                },
            },
            EventEntry {
                index: 1,
                user_id: user_b.clone(),
                log: Event::AddNode {
                    node_id: node_b.clone(),
                    content: Content::String("string".into()),
                },
            },
            EventEntry {
                index: 1,
                user_id: user_b.clone(),
                log: Event::PatchChildren {
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
                log: Event::AddCard {
                    card_id: card_id.clone(),
                    node_id: node_a.clone(),
                },
            },
        ]);

        let mut kernel = Kernel::new(repository);
        kernel.process();

        assert_eq!(kernel.snapshot.nodes.len(), 2);
        assert_eq!(kernel.snapshot.owners.len(), 1);
        assert_eq!(
            kernel.references.0,
            [(node_b.clone(), [node_a.clone()].into_iter().collect()),]
                .into_iter()
                .collect()
        );
        assert_eq!(
            kernel.db.hir(card_id),
            Ok(Arc::new(WithMeta {
                meta: Meta {
                    attrs: vec![],
                    id: 0,
                    span: 0..0
                },
                value: Expr::Apply {
                    function: WithMeta {
                        meta: Meta {
                            attrs: vec![],
                            id: 0,
                            span: 0..0
                        },
                        value: HirType::Function {
                            parameter: Box::new(WithMeta {
                                meta: Meta {
                                    attrs: vec![],
                                    id: 0,
                                    span: 0..0
                                },
                                value: HirType::String
                            }),
                            body: Box::new(WithMeta {
                                meta: Meta {
                                    attrs: vec![],
                                    id: 0,
                                    span: 0..0
                                },
                                value: HirType::Number
                            }),
                        }
                    },
                    link_name: LinkName::None,
                    arguments: vec![WithMeta {
                        meta: Meta {
                            attrs: vec![],
                            id: 0,
                            span: 0..0
                        },
                        value: Expr::Literal(Literal::String("string".into()))
                    }]
                }
            }))
        );
    }
}
