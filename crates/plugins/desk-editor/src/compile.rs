use std::collections::HashMap;

use deskc_ids::{CardId, NodeId};
use dkernel::prelude::*;

pub struct Cards {
    pub cards: HashMap<CardId, NodeId>,
}

fn card_id_type() -> Type {
    Type::Label {
        label: "desk-editor card-id".into(),
        item: Box::new(Type::Vector(Box::new(Type::Number))),
    }
}

// fn uuid_from_expr(_expr: &Expr) -> Uuid {
//     todo!()
// }

impl State for Cards {
    fn handle_event(&mut self, _snapshot: &Snapshot, event: &Event) {
        match event {
            Event::PatchAttribute { node_id: _, patch } => match patch {
                dkernel_components::patch::AttributePatch::Update { key, value: _ } => {
                    if *key == card_id_type() {
                        // let card_id = CardId();
                        // self.cards.insert(card_id, node_id.clone());
                    }
                }
                dkernel_components::patch::AttributePatch::Remove { key: _ } => todo!(),
            },
            Event::AddSnapshot {
                index: _,
                snapshot: _,
            } => {}
            Event::RemoveNode { node_id: _ } => {}
            _ => {}
        }
    }
}
