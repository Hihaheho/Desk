mod ast;
mod node;

use std::sync::Arc;

use ast::ast;
use node::node;

use components::{code::Code, event::Event, flat_node::FlatNode, node::Node};
use deskc_ids::NodeId;

use crate::query_error::QueryError;

#[salsa::query_group(KernelStorage)]
pub trait NodeQueries {
    #[salsa::input]
    fn flat_node(&self, id: NodeId) -> Arc<FlatNode>;
    fn node(&self, id: NodeId) -> Arc<Node>;
    fn ast(&self, id: NodeId) -> Result<Code, QueryError>;
}

#[salsa::database(KernelStorage)]
#[derive(Default)]
pub struct Nodes {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for Nodes {}

impl Nodes {
    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::CreateNode { node_id, content } => {
                self.set_flat_node(node_id.clone(), Arc::new(FlatNode::new(content.clone())));
            }
            Event::PatchContent { node_id, patch } => {
                let mut flat_node = self.flat_node(node_id.clone()).as_ref().clone();
                flat_node.patch_content(patch);
                self.set_flat_node(node_id.clone(), Arc::new(flat_node));
            }
            Event::PatchOperand { node_id, patch } => {
                let mut flat_node = self.flat_node(node_id.clone()).as_ref().clone();
                flat_node.patch_children(patch);
                self.set_flat_node(node_id.clone(), Arc::new(flat_node));
            }
            Event::PatchAttribute { node_id, patch } => {
                let mut flat_node = self.flat_node(node_id.clone()).as_ref().clone();
                flat_node.patch_attribute(patch);
                self.set_flat_node(node_id.clone(), Arc::new(flat_node));
            }
            Event::AddSnapshot { .. } => todo!(),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use components::{
        content::Content,
        patch::{AttributePatch, ContentPatch, OperandPatch},
    };
    use deskc_ty::Type;

    use super::*;

    #[test]
    fn add_node() {
        let mut db = Nodes::default();
        let node_id = NodeId::new();

        db.handle_event(&Event::CreateNode {
            node_id: node_id.clone(),
            content: Content::String("a".into()),
        });

        assert_eq!(
            *db.flat_node(node_id),
            FlatNode::new(Content::String("a".into()))
        );
    }

    #[test]
    fn patch_content() {
        let mut db = Nodes::default();
        let node_id = handle_add_node(&mut db);

        db.handle_event(&Event::PatchContent {
            node_id: node_id.clone(),
            patch: ContentPatch::Replace(Content::String("b".into())),
        });

        assert_eq!(db.flat_node(node_id).content, Content::String("b".into()));
    }

    #[test]
    fn patch_children() {
        let mut db = Nodes::default();
        let node_id = handle_add_node(&mut db);
        let node_a = NodeId::new();

        db.handle_event(&Event::PatchOperand {
            node_id: node_id.clone(),
            patch: OperandPatch::Insert {
                index: 0,
                node_id: node_a.clone(),
            },
        });

        assert_eq!(db.flat_node(node_id).operands, vec![node_a]);
    }

    #[test]
    fn patch_attribute() {
        let mut db = Nodes::default();
        let node_id = handle_add_node(&mut db);

        db.handle_event(&Event::PatchAttribute {
            node_id: node_id.clone(),
            patch: AttributePatch::Update {
                key: Type::Real,
                value: 0.into(),
            },
        });

        assert_eq!(
            db.flat_node(node_id).attributes,
            vec![(Type::Real, 0.into())].into_iter().collect()
        );
    }

    fn handle_add_node(db: &mut Nodes) -> NodeId {
        let node_id = NodeId::new();
        db.handle_event(&Event::CreateNode {
            node_id: node_id.clone(),
            content: Content::String("a".into()),
        });
        node_id
    }
}
