mod ast;
mod node;

use std::sync::Arc;

use ast::ast;
use deskc_ast::{expr::Expr, span::WithSpan};
use node::node;

use components::{event::Event, flat_node::FlatNode, node::Node};
use deskc_ids::NodeId;

use crate::query_result::QueryResult;

#[salsa::query_group(KernelStorage)]
pub trait NodeQueries {
    #[salsa::input]
    fn flat_node(&self, id: NodeId) -> Arc<FlatNode>;
    fn node(&self, id: NodeId) -> Arc<Node>;
    fn ast(&self, id: NodeId) -> QueryResult<WithSpan<Expr>>;
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
            Event::AddNode {
                parent,
                node_id,
                content,
            } => {
                self.set_flat_node(
                    node_id.clone(),
                    Arc::new(FlatNode::new(content.clone()).parent(parent.clone())),
                );
            }
            Event::PatchContent { node_id, patch } => {
                let mut flat_node = self.flat_node(node_id.clone()).as_ref().clone();
                flat_node.patch_content(patch);
                self.set_flat_node(node_id.clone(), Arc::new(flat_node));
            }
            Event::PatchOperands { node_id, patch } => {
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
        patch::{AttributePatch, ContentPatch, OperandsPatch},
    };
    use deskc_hir::expr::{Expr, Literal};
    use deskc_types::Type;

    use super::*;

    #[test]
    fn add_node() {
        let mut db = Nodes::default();
        let node_id = NodeId::new();
        let parent = Some(NodeId::new());

        db.handle_event(&Event::AddNode {
            parent: parent.clone(),
            node_id: node_id.clone(),
            content: Content::String("a".into()),
        });

        assert_eq!(
            *db.flat_node(node_id),
            FlatNode::new(Content::String("a".into())).parent(parent)
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

        db.handle_event(&Event::PatchOperands {
            node_id: node_id.clone(),
            patch: OperandsPatch::Insert {
                index: 0,
                node: node_a.clone(),
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
                key: Type::Number,
                value: Box::new(Expr::Literal(Literal::Integer(0))),
            },
        });

        assert_eq!(
            db.flat_node(node_id).attributes,
            vec![(Type::Number, Expr::Literal(Literal::Integer(0)))]
                .into_iter()
                .collect()
        );
    }

    fn handle_add_node(db: &mut Nodes) -> NodeId {
        let node_id = NodeId::new();
        db.handle_event(&Event::AddNode {
            parent: None,
            node_id: node_id.clone(),
            content: Content::String("a".into()),
        });
        node_id
    }
}
