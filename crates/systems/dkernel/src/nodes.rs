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
pub struct Hirs {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for Hirs {}

impl Hirs {
    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::AddNode {
                node_id,
                content,
                file_id,
            } => {
                self.set_flat_node(
                    node_id.clone(),
                    Arc::new(FlatNode {
                        file_id: file_id.clone(),
                        content: content.clone(),
                        children: Default::default(),
                        attributes: Default::default(),
                    }),
                );
            }
            Event::PatchContent { node_id, patch } => {
                let mut flat_node = self.flat_node(node_id.clone()).as_ref().clone();
                flat_node.patch_content(patch);
                self.set_flat_node(node_id.clone(), Arc::new(flat_node));
            }
            Event::PatchChildren { node_id, patch } => {
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
        flat_node::{Attributes, Children},
        patch::{AttributePatch, ChildrenPatch, ContentPatch},
    };
    use deskc_hir::expr::{Expr, Literal};
    use deskc_ids::FileId;
    use deskc_types::Type;

    use super::*;

    #[test]
    fn add_node() {
        let mut db = Hirs::default();
        let node_id = NodeId::new();
        let file_id = FileId::new();

        db.handle_event(&Event::AddNode {
            node_id: node_id.clone(),
            file_id: file_id.clone(),
            content: Content::String("a".into()),
        });

        assert_eq!(
            *db.flat_node(node_id),
            FlatNode {
                file_id,
                content: Content::String("a".into()),
                children: Children::default(),
                attributes: Attributes::default(),
            }
        );
    }

    #[test]
    fn patch_content() {
        let mut db = Hirs::default();
        let node_id = handle_add_node(&mut db);

        db.handle_event(&Event::PatchContent {
            node_id: node_id.clone(),
            patch: ContentPatch::Replace(Content::String("b".into())),
        });

        assert_eq!(db.flat_node(node_id).content, Content::String("b".into()));
    }

    #[test]
    fn patch_children() {
        let mut db = Hirs::default();
        let node_id = handle_add_node(&mut db);
        let node_a = NodeId::new();

        db.handle_event(&Event::PatchChildren {
            node_id: node_id.clone(),
            patch: ChildrenPatch::Insert {
                index: 0,
                node: node_a.clone(),
            },
        });

        assert_eq!(db.flat_node(node_id).children, vec![node_a]);
    }

    #[test]
    fn patch_attribute() {
        let mut db = Hirs::default();
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

    fn handle_add_node(db: &mut Hirs) -> NodeId {
        let node_id = NodeId::new();
        db.handle_event(&Event::AddNode {
            node_id: node_id.clone(),
            file_id: FileId::new(),
            content: Content::String("a".into()),
        });
        node_id
    }
}
