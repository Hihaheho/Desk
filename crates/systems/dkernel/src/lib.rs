mod cards;
mod history;
mod queries;
mod query_result;
mod repository;
mod snapshot;

use cards::Cards;
use history::History;
use queries::KernelStorage;
use repository::Repository;
use snapshot::Snapshot;

pub struct Kernel {
    repository: Box<dyn Repository>,
    db: KernelDatabase,
    pub snapshot: Snapshot,
    pub cards: Cards,
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
            cards: Default::default(),
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
    use deskc_ids::{CardId, UserId};
    use deskc_types::Type;
    use dkernel_card::{
        content::Content, flat_node::NodeRef, node::NodeId, patch::ChildrenPatch,
    };
    use uuid::Uuid;

    use crate::repository::{Log, LogEntry};

    use super::*;

    #[mry::mry]
    #[derive(Default)]
    pub struct TestRepository {}

    #[mry::mry]
    impl Repository for TestRepository {
        fn poll(&mut self) -> Vec<repository::LogEntry> {
            todo!()
        }
        fn commit(&mut self, log: repository::Log) {
            todo!()
        }
        fn add_owner(&mut self, user_id: UserId) {
            todo!()
        }
        fn remove_owner(&mut self, user_id: UserId) {
            todo!()
        }
    }

    fn integration() {
        let mut repository = TestRepository::default();

        let user_a = UserId("a".into());
        let user_b = UserId("b".into());
        let node_a = NodeId(Uuid::new_v4());
        let node_b = NodeId(Uuid::new_v4());

        repository.mock_poll().returns(vec![
            LogEntry {
                index: 0,
                user_id: user_a.clone(),
                log: Log::AddOwner {
                    user_id: user_a.clone(),
                },
            },
            LogEntry {
                index: 0,
                user_id: user_b.clone(),
                log: Log::AddOwner {
                    user_id: user_b.clone(),
                },
            },
            LogEntry {
                index: 0,
                user_id: user_a.clone(),
                log: Log::AddNode {
                    node_id: node_a.clone(),
                    content: Content::Apply(Type::Function {
                        parameters: vec![Type::String],
                        body: Box::new(Type::Number),
                    }),
                },
            },
            LogEntry {
                index: 1,
                user_id: user_b.clone(),
                log: Log::AddNode {
                    node_id: node_b.clone(),
                    content: Content::String("string".into()),
                },
            },
            LogEntry {
                index: 1,
                user_id: user_b.clone(),
                log: Log::PatchChildren {
                    node_id: node_a.clone(),
                    patch: ChildrenPatch::Insert {
                        index: 0,
                        node: NodeRef::Node(node_b.clone()),
                    },
                },
            },
        ]);

        let mut kernel = Kernel::new(repository);
        kernel.process();

        assert_eq!(kernel.snapshot.nodes.len(), 2);
        assert_eq!(kernel.snapshot.owners.len(), 1);
        assert_eq!(
            kernel.cards.cards,
            vec![(
                CardId(node_a.0.clone()),
                vec![node_a.clone(), node_b.clone()].into_iter().collect()
            ),]
            .into_iter()
            .collect()
        );
        assert_eq!(kernel.cards.cards.len(), 1);
    }
}
