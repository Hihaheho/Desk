use std::collections::HashMap;

use deskc_ids::{CardId, NodeId};
use deskc_macros::ty;
use dworkspace::prelude::*;
use once_cell::sync::Lazy;

pub struct Cards {
    pub cards: HashMap<CardId, NodeId>,
}

const CARD_ID_TYPE: Lazy<Type> = Lazy::new(|| ty!("@`desk-editor card-id` 'string"));

impl State for Cards {
    fn handle_event(&mut self, _snapshot: &Projection, event: &Event) {
        match &event.payload {
            EventPayload::PatchAttribute { node_id: _, patch } => match patch {
                dworkspace_codebase::patch::AttributePatch::Update { key, value: _ } => {
                    if *key == *CARD_ID_TYPE {
                        // let card_id = CardId();
                        // self.cards.insert(card_id, node_id.clone());
                    }
                }
                dworkspace_codebase::patch::AttributePatch::Remove { key: _ } => todo!(),
            },
            EventPayload::AddSnapshot {
                index: _,
                snapshot: _,
            } => {}
            EventPayload::RemoveNode { node_id: _ } => {}
            _ => {}
        }
    }
}
