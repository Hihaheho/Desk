use std::collections::{HashMap, HashSet};

use crate::event::Event;
use crate::patch::FilePatch;
use crate::rules::{Rules, SpaceOperation};
use crate::user::UserId;
use crate::{file::File, flat_node::FlatNode};
use deskc_ids::{CardId, FileId, NodeId};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Snapshot {
    pub owners: HashSet<UserId>,
    // flat nodes are owned by hirs db
    pub flat_nodes: HashMap<NodeId, FlatNode>,
    pub files: HashMap<FileId, File>,
    pub rules: Rules<SpaceOperation>,
    pub card_files: HashMap<CardId, FileId>,
}

impl Snapshot {
    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::AddOwner { user_id } => {
                self.owners.insert(user_id.clone());
            }
            Event::RemoveOwner { user_id: _ } => todo!(),
            Event::AddNode {
                node_id,
                file_id,
                content,
            } => {
                self.flat_nodes.insert(
                    node_id.clone(),
                    FlatNode::new(file_id.clone(), content.clone()),
                );
            }
            Event::RemoveNode { node_id } => {
                self.flat_nodes.remove(node_id);
            }
            Event::PatchContent { node_id, patch } => {
                self.flat_nodes
                    .get_mut(node_id)
                    .unwrap()
                    .patch_content(patch);
            }
            Event::PatchChildren { node_id, patch } => {
                self.flat_nodes
                    .get_mut(node_id)
                    .unwrap()
                    .patch_children(patch);
            }
            Event::PatchAttribute { node_id, patch } => {
                self.flat_nodes
                    .get_mut(node_id)
                    .unwrap()
                    .patch_attribute(patch);
            }
            Event::AddSnapshot {
                index: _,
                snapshot: _,
            } => todo!(),
            Event::AddFile(file_id) => {
                self.files.insert(file_id.clone(), File::default());
            }
            Event::DeleteFile(file_id) => {
                self.files.remove(file_id);
            }
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
    use crate::{content::Content, rules::NodeOperation};

    use super::*;

    #[test]
    fn add_owner() {
        let mut snapshot = Snapshot::default();
        snapshot.handle_event(&Event::AddOwner {
            user_id: UserId("a".into()),
        });
        assert_eq!(
            snapshot.owners,
            vec![UserId("a".into())].into_iter().collect()
        )
    }

    #[test]
    fn add_node() {
        let mut snapshot = Snapshot::default();
        let node_id = handle_add_node(&mut snapshot);
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
        let node_id = handle_add_node(&mut snapshot);
        snapshot.handle_event(&Event::RemoveNode { node_id });

        assert_eq!(snapshot.flat_nodes, HashMap::default())
    }

    #[test]
    fn add_file() {
        let mut snapshot = Snapshot::default();
        let file_id = FileId::new();
        snapshot.handle_event(&Event::AddFile(file_id.clone()));

        assert_eq!(
            snapshot.files,
            [(file_id, File::default())].into_iter().collect()
        );
    }

    #[test]
    fn remove_file() {
        let mut snapshot = Snapshot::default();
        let file_a = FileId::new();
        let file_b = FileId::new();
        snapshot.handle_event(&Event::AddFile(file_a.clone()));
        snapshot.handle_event(&Event::AddFile(file_b.clone()));
        snapshot.handle_event(&Event::DeleteFile(file_b));
        assert_eq!(
            snapshot.files,
            [(file_a, File::default())].into_iter().collect()
        );
    }

    #[test]
    fn patch_file_update_rules() {
        let mut snapshot = Snapshot::default();
        let file_id = FileId::new();
        snapshot.handle_event(&Event::AddFile(file_id.clone()));
        snapshot.handle_event(&Event::PatchFile {
            file_id: file_id.clone(),
            patch: FilePatch::UpdateRules {
                rules: Rules {
                    default: [NodeOperation::AddCard].into_iter().collect(),
                    users: Default::default(),
                },
            },
        });

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
        snapshot.handle_event(&Event::UpdateRule {
            rules: Rules {
                default: [SpaceOperation::AddFile].into_iter().collect(),
                users: Default::default(),
            },
        });

        assert_eq!(
            snapshot.rules,
            Rules {
                default: [SpaceOperation::AddFile].into_iter().collect(),
                users: Default::default(),
            }
        );
    }

    fn handle_add_node(snapshot: &mut Snapshot) -> NodeId {
        let node_id = NodeId::new();
        let event = Event::AddNode {
            node_id: node_id.clone(),
            file_id: FileId::default(),
            content: Content::String("a".into()),
        };
        snapshot.handle_event(&event);
        node_id
    }
}
