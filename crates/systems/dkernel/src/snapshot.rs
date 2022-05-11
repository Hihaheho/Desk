use std::collections::{HashMap, HashSet};

use components::patch::FilePatch;
use components::rules::{Rules, SpaceOperation};
use components::{file::File, flat_node::FlatNode};
use deskc_ids::{CardId, FileId, NodeId, UserId};

use crate::hirs::HirQueries;
use crate::{event::Event, hirs::Hirs};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Snapshot {
    pub owners: HashSet<UserId>,
    // flat nodes are owned by hirs db
    pub flat_nodes: HashMap<NodeId, FlatNode>,
    pub node_files: HashMap<NodeId, FileId>,
    pub cards: HashMap<CardId, NodeId>,
    pub files: HashMap<FileId, File>,
    // None is the rare and short-lived.
    pub default_file: Option<FileId>,
    pub rules: Rules<SpaceOperation>,
    pub card_files: HashMap<CardId, FileId>,
}

impl Snapshot {
    pub fn handle_event(&mut self, hirs: &Hirs, event: &Event) {
        let mut derive_from_hirs = |node_id: &NodeId| {
            self.flat_nodes
                .insert(node_id.clone(), (*hirs.flat_node(node_id.clone())).clone());
        };
        match event {
            Event::AddOwner { user_id } => {
                self.owners.insert(user_id.clone());
            }
            Event::RemoveOwner { user_id: _ } => todo!(),
            Event::AddNode { node_id, .. } => derive_from_hirs(node_id),
            Event::RemoveNode { node_id } => {
                self.flat_nodes.remove(node_id);
            }
            Event::PatchContent { node_id, .. } => derive_from_hirs(node_id),
            Event::PatchChildren { node_id, .. } => derive_from_hirs(node_id),
            Event::PatchAttribute { node_id, .. } => derive_from_hirs(node_id),
            Event::AddSnapshot {
                index: _,
                snapshot: _,
            } => todo!(),
            Event::AddFile(file_id) => {
                self.files.insert(file_id.clone(), File::default());
                if self.default_file.is_none() {
                    self.default_file = Some(file_id.clone());
                }
            }
            Event::DeleteFile(file_id) => {
                self.files.remove(file_id);
            }
            Event::AddCard { card_id, node_id } => {
                self.cards.insert(card_id.clone(), node_id.clone());
            }
            Event::RemoveCard { card_id: _ } => todo!(),
            Event::UpdateRule { rules } => {
                self.rules = rules.clone();
            }
            Event::PatchFile { file_id, patch } => {
                match patch {
                    FilePatch::UpdateRules { rules } => {
                        // unwrap is safe because audit checks that the file exists
                        self.files.get_mut(file_id).unwrap().rules = rules.clone()
                    }
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use components::{content::Content, patch::FilePatch, rules::NodeOperation};
    use uuid::Uuid;

    use super::*;

    #[test]
    fn add_owner() {
        let mut snapshot = Snapshot::default();
        snapshot.handle_event(
            &Default::default(),
            &Event::AddOwner {
                user_id: UserId("a".into()),
            },
        );
        assert_eq!(
            snapshot.owners,
            vec![UserId("a".into())].into_iter().collect()
        )
    }

    #[test]
    fn add_node() {
        let mut snapshot = Snapshot::default();
        let node_id = handle_add_node(&mut Default::default(), &mut snapshot);
        assert_eq!(
            snapshot.flat_nodes,
            [(
                node_id,
                FlatNode {
                    file_id: FileId::default(),
                    content: Content::String("a".into()),
                    children: vec![],
                    attributes: [].into_iter().collect(),
                }
            )]
            .into_iter()
            .collect()
        )
    }

    #[test]
    fn remove_node() {
        let mut snapshot = Snapshot::default();
        let mut hirs = Hirs::default();
        let node_id = handle_add_node(&mut hirs, &mut snapshot);
        snapshot.handle_event(&Default::default(), &Event::RemoveNode { node_id });

        assert_eq!(snapshot.flat_nodes, HashMap::default())
    }

    #[test]
    fn add_card() {
        let mut snapshot = Snapshot::default();
        let node_id = handle_add_node(&mut Default::default(), &mut snapshot);
        let card_id = CardId(Uuid::new_v4());
        snapshot.handle_event(
            &Default::default(),
            &Event::AddCard {
                card_id: card_id.clone(),
                node_id: node_id.clone(),
            },
        );

        assert_eq!(snapshot.cards, [(card_id, node_id)].into_iter().collect());
    }

    #[test]
    fn add_file() {
        let mut snapshot = Snapshot::default();
        let file_id = FileId(Uuid::new_v4());
        snapshot.handle_event(&Default::default(), &Event::AddFile(file_id.clone()));

        assert_eq!(
            snapshot.files,
            [(file_id.clone(), File::default())].into_iter().collect()
        );
        assert_eq!(snapshot.default_file, Some(file_id));
    }

    #[test]
    fn add_file_some_default() {
        let mut snapshot = Snapshot::default();
        let default_file = FileId(Uuid::new_v4());
        let file_id = FileId(Uuid::new_v4());
        snapshot.handle_event(&Default::default(), &Event::AddFile(default_file.clone()));
        snapshot.handle_event(&Default::default(), &Event::AddFile(file_id.clone()));

        assert_eq!(
            snapshot.files,
            [
                (default_file.clone(), File::default()),
                (file_id, File::default())
            ]
            .into_iter()
            .collect()
        );
        assert_eq!(snapshot.default_file, Some(default_file));
    }

    #[test]
    fn remove_file() {
        let mut snapshot = Snapshot::default();
        let file_a = FileId(Uuid::new_v4());
        let file_b = FileId(Uuid::new_v4());
        snapshot.handle_event(&Default::default(), &Event::AddFile(file_a.clone()));
        snapshot.handle_event(&Default::default(), &Event::AddFile(file_b.clone()));
        snapshot.handle_event(&Default::default(), &Event::DeleteFile(file_b));
        assert_eq!(
            snapshot.files,
            [(file_a, File::default())].into_iter().collect()
        );
    }

    #[test]
    fn patch_file_update_rules() {
        let mut snapshot = Snapshot::default();
        let file_id = FileId(Uuid::new_v4());
        snapshot.handle_event(&Default::default(), &Event::AddFile(file_id.clone()));
        snapshot.handle_event(
            &Default::default(),
            &Event::PatchFile {
                file_id: file_id.clone(),
                patch: FilePatch::UpdateRules {
                    rules: Rules {
                        default: [NodeOperation::AddCard].into_iter().collect(),
                        users: Default::default(),
                    },
                },
            },
        );

        assert_eq!(
            snapshot.files,
            [(
                file_id,
                File {
                    rules: Rules {
                        default: [NodeOperation::AddCard].into_iter().collect(),
                        users: Default::default(),
                    },
                    ..File::default()
                }
            )]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn update_rule() {
        let mut snapshot = Snapshot::default();
        snapshot.handle_event(
            &Default::default(),
            &Event::UpdateRule {
                rules: Rules {
                    default: [SpaceOperation::AddFile].into_iter().collect(),
                    users: Default::default(),
                },
            },
        );

        assert_eq!(
            snapshot.rules,
            Rules {
                default: [SpaceOperation::AddFile].into_iter().collect(),
                users: Default::default(),
            }
        );
    }

    fn handle_add_node(hirs: &mut Hirs, snapshot: &mut Snapshot) -> NodeId {
        let node_id = NodeId(Uuid::new_v4());
        let event = Event::AddNode {
            node_id: node_id.clone(),
            file_id: FileId(Uuid::new_v4()),
            content: Content::String("a".into()),
        };
        hirs.handle_event(&event);
        snapshot.handle_event(hirs, &event);
        node_id
    }
}
