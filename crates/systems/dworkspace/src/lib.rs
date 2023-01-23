mod audit;
mod descendants;
mod error;
mod history;
mod loop_detector;
mod nodes;
pub mod prelude;
pub mod query_error;
mod references;
pub mod repository;
pub mod state;

use std::{any::TypeId, collections::HashMap};

use bevy_ecs::prelude::Component;
use components::{event::Event, projection::Projection};
use deskc_ids::NodeId;
use history::History;
use loop_detector::LoopDetector;
use nodes::Nodes;
use parking_lot::Mutex;
use repository::Repository;
use state::State;

#[derive(Component)]
pub struct Workspace {
    repository: Box<dyn Repository + Send + Sync + 'static>,
    // salsa database is not Sync
    nodes: Mutex<Nodes>,
    // salsa database is not Sync
    references: Mutex<references::References>,
    loop_detector: LoopDetector,
    pub snapshot: Projection,
    history: History,
    states: HashMap<TypeId, Box<dyn State + Send + Sync + 'static>>,
}

impl Workspace {
    pub fn new(repository: impl Repository + Send + Sync + 'static) -> Self {
        Self {
            repository: Box::new(repository),
            nodes: Default::default(),
            references: Default::default(),
            loop_detector: Default::default(),
            snapshot: Default::default(),
            history: Default::default(),
            states: Default::default(),
        }
    }

    pub fn commit(&mut self, event: Event) {
        self.repository.commit(event);
    }

    pub fn process(&mut self) {
        let events = self.repository.poll();
        for event in events {
            if self.audit(&event).is_ok() {
                self.handle_event(&event);
            }
        }
    }

    fn handle_event(&mut self, event: &Event) {
        self.nodes.lock().handle_event(event);
        self.references.lock().handle_event(&self.snapshot, event);
        self.history.handle_event(&self.snapshot, event);
        for state in self.states.values_mut() {
            state.handle_event(&self.snapshot, event);
        }
        self.loop_detector.handle_event(&self.snapshot, event);
        // This must be last for using the previous snapshot above
        self.snapshot.handle_event(event);
    }

    pub fn add_state<T: State + Send + Sync + 'static>(&mut self, state: T) {
        self.states.insert(TypeId::of::<T>(), Box::new(state));
    }

    pub fn get_state<T: State + Send + Sync + 'static>(&self) -> Option<&T> {
        self.states
            .get(&TypeId::of::<T>())
            .map(|state| state.as_any().downcast_ref::<T>().unwrap())
    }

    pub fn get_state_mut<T: State + Send + Sync + 'static>(&mut self) -> Option<&mut T> {
        self.states
            .get_mut(&TypeId::of::<T>())
            .map(|state| state.as_any_mut().downcast_mut::<T>().unwrap())
    }

    pub fn top_level_nodes(&self) -> Vec<NodeId> {
        self.references.lock().top_level_nodes().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use components::code::Code;
    use components::event::{Event, EventId, EventPayload};
    use components::patch::OperandPosition;
    use components::rules::{NodeOperation, Rules, SpaceOperation};
    use components::user::UserId;
    use components::{content::Content, patch::OperandPatch};
    use deskc_ast::remove_span::replace_node_id_to_default;
    use deskc_ast::ty::Function;
    use deskc_ast::{
        expr::{Expr, Literal},
        meta::WithMeta,
        ty::Type as AstType,
    };
    use deskc_ids::{LinkName, NodeId};
    use nodes::NodeQueries;

    use crate::repository::TestRepository;

    use super::*;

    #[mry::mry]
    #[derive(Default)]
    pub struct TestState {}

    #[mry::mry]
    impl State for TestState {
        fn handle_event(&mut self, _snapshot: &Projection, _: &Event) {
            panic!()
        }
    }

    #[test]
    fn integration() {
        let mut repository = TestRepository::default();

        let user_a = UserId::new();
        let user_b = UserId::new();
        let node_a = NodeId::new();
        let node_b = NodeId::new();

        let add_owner = Event {
            id: EventId::new(),
            user_id: user_a.clone(),
            payload: EventPayload::AddOwner {
                user_id: user_a.clone(),
            },
        };

        repository.mock_poll().returns(vec![
            add_owner.clone(),
            Event {
                id: EventId::new(),
                user_id: user_b.clone(),
                payload: EventPayload::AddOwner {
                    user_id: user_b.clone(),
                },
            },
            Event {
                id: EventId::new(),
                user_id: user_a.clone(),
                payload: EventPayload::UpdateSpaceRules {
                    rules: Rules {
                        default: [SpaceOperation::CreateNode].into_iter().collect(),
                        users: Default::default(),
                    },
                },
            },
            Event {
                id: EventId::new(),
                user_id: user_a.clone(),
                payload: EventPayload::CreateNode {
                    node_id: node_a.clone(),
                    content: Content::Apply {
                        link_name: Default::default(),
                    },
                },
            },
            Event {
                id: EventId::new(),
                user_id: user_a.clone(),
                payload: EventPayload::UpdateNodeRules {
                    node_id: node_a.clone(),
                    rules: Rules {
                        default: [NodeOperation::InsertOperand].into_iter().collect(),
                        users: Default::default(),
                    },
                },
            },
            Event {
                id: EventId::new(),
                user_id: user_b.clone(),
                payload: EventPayload::CreateNode {
                    node_id: node_b.clone(),
                    content: Content::String("string".into()),
                },
            },
            Event {
                id: EventId::new(),
                user_id: user_b,
                payload: EventPayload::PatchOperand {
                    node_id: node_a.clone(),
                    patch: OperandPatch::Insert {
                        position: OperandPosition::First,
                        node_id: node_b,
                    },
                },
            },
        ]);

        let mut test_state = TestState::default();
        test_state.mock_handle_event(mry::Any, mry::Any).returns(());

        let mut kernel = Workspace::new(repository);
        kernel.add_state(test_state);
        kernel.process();

        assert_eq!(kernel.snapshot.flat_nodes.len(), 2);
        assert_eq!(kernel.snapshot.owners.len(), 1);
        let mut ast = if let Code::Ast(ast) = kernel.nodes.lock().ast(node_a).unwrap() {
            ast.as_ref().clone()
        } else {
            panic!();
        };
        replace_node_id_to_default(&mut ast);
        assert_eq!(
            ast,
            WithMeta {
                meta: Default::default(),
                value: Expr::Apply {
                    function: WithMeta {
                        meta: Default::default(),
                        value: AstType::Function(Box::new(Function {
                            parameter: WithMeta {
                                meta: Default::default(),
                                value: AstType::String
                            },
                            body: WithMeta {
                                meta: Default::default(),
                                value: AstType::Real
                            },
                        }))
                    },
                    link_name: LinkName::None,
                    arguments: vec![WithMeta {
                        meta: Default::default(),
                        value: Expr::Literal(Literal::String("string".into()))
                    }]
                }
            }
        );

        kernel
            .get_state_mut::<TestState>()
            .unwrap()
            .mock_handle_event(mry::Any, mry::Any)
            .assert_called(6);

        // asserts handle_event was called with unprocessed snapshot
        kernel
            .get_state_mut::<TestState>()
            .unwrap()
            .mock_handle_event(Projection::default(), add_owner)
            .assert_called(1);
    }
}
