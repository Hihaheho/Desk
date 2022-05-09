use std::collections::{HashMap, HashSet};

use deskc_ids::{CardId, FileId, UserId};
use dkernel_card::{file::File, flat_node::FlatNode, node::NodeId};

use crate::event::{Event, EventEntry};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Snapshot {
    pub owners: HashSet<UserId>,
    pub nodes: HashMap<NodeId, FlatNode>,
    pub cards: HashMap<CardId, NodeId>,
    pub files: HashMap<FileId, File>,
    pub default_file: Option<FileId>,
    pub card_files: HashMap<CardId, FileId>,
}

impl Snapshot {
    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::AddOwner { user_id } => {
                self.owners.insert(user_id.clone());
            }
            Event::RemoveOwner { user_id } => todo!(),
            Event::AddNode { node_id, content } => {
                self.nodes.insert(
                    node_id.clone(),
                    FlatNode {
                        content: content.clone(),
                        children: Default::default(),
                        attributes: Default::default(),
                    },
                );
            }
            Event::RemoveNode(_) => todo!(),
            Event::PatchContent { node_id, patch } => todo!(),
            Event::PatchChildren { node_id, patch } => todo!(),
            Event::PatchAttribute { node_id, patch } => todo!(),
            Event::AddSnapshot { index, snapshot } => todo!(),
            Event::AddFile(_) => todo!(),
            Event::DeleteFile(_) => todo!(),
            Event::AddCard { card_id, node_id } => {
                self.cards.insert(card_id.clone(), node_id.clone());
            }
            Event::RemoveCard(_) => todo!(),
        }
    }
    pub fn allowed_log_entry(&self, entry: &EventEntry) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use dkernel_card::content::Content;
    use uuid::Uuid;

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
        let node_id = NodeId(Uuid::new_v4());
        snapshot.handle_event(&Event::AddNode {
            node_id: node_id.clone(),
            content: Content::String("a".into()),
        });
        assert_eq!(
            snapshot.nodes,
            [(
                node_id,
                FlatNode {
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
    fn add_card() {
        let mut snapshot = Snapshot::default();
        let node_id = handle_add_node(&mut snapshot);
        let card_id = CardId(Uuid::new_v4());
        snapshot.handle_event(&Event::AddCard {
            card_id: card_id.clone(),
            node_id: node_id.clone(),
        });

        assert_eq!(snapshot.cards, [(card_id, node_id)].into_iter().collect());
    }

    fn handle_add_node(snapshot: &mut Snapshot) -> NodeId {
        let node_id = NodeId(Uuid::new_v4());
        snapshot.handle_event(&Event::AddNode {
            node_id: node_id.clone(),
            content: Content::String("a".into()),
        });
        node_id
    }
}
