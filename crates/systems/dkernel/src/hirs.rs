mod apply_patch;
mod ast;
mod build_ast;
mod flat_node;
mod hir;
mod typed_amir;

use std::sync::Arc;

use ast::ast;
use build_ast::build_ast;
use flat_node::flat_node;
use hir::hir;

use apply_patch::*;
use deskc_ast::span::Spanned;
use deskc_hir::meta::WithMeta;
use deskc_ids::{CardId, NodeId};
use dkernel_card::{
    content::Content,
    flat_node::{Attributes, FlatNode, NodeRef},
    node::Node,
};

use crate::{event::Event, query_result::QueryResult};

#[salsa::query_group(KernelStorage)]
pub trait HirQueries {
    #[salsa::input]
    fn content(&self, id: NodeId) -> Content;
    #[salsa::input]
    fn children(&self, id: NodeId) -> Vec<NodeRef>;
    #[salsa::input]
    fn attributes(&self, id: NodeId) -> Attributes;
    fn flat_node(&self, id: NodeId) -> Arc<FlatNode>;
    fn build_ast(&self, id: NodeId) -> QueryResult<Node>;
    #[salsa::input]
    fn node_id(&self, id: CardId) -> NodeId;
    fn ast(&self, id: CardId) -> QueryResult<Spanned<deskc_ast::expr::Expr>>;
    fn hir(&self, id: CardId) -> QueryResult<WithMeta<deskc_hir::expr::Expr>>;
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
                node_id, content, ..
            } => {
                self.set_content(node_id.clone(), content.clone());
                self.set_children(node_id.clone(), vec![]);
                self.set_attributes(node_id.clone(), Attributes::default());
            }
            Event::PatchContent { node_id, patch } => match patch {
                dkernel_card::patch::ContentPatch::Replace(content) => {
                    self.set_content(node_id.clone(), content.clone());
                }
                patch => {
                    self.set_content(
                        node_id.clone(),
                        self.content(node_id.clone()).apply_patch(patch),
                    );
                }
            },
            Event::PatchChildren { node_id, patch } => {
                self.set_children(
                    node_id.clone(),
                    self.children(node_id.clone()).apply_patch(patch),
                );
            }
            Event::PatchAttribute { node_id, patch } => {
                self.set_attributes(
                    node_id.clone(),
                    self.attributes(node_id.clone()).apply_patch(patch),
                );
            }
            Event::AddSnapshot { .. } => todo!(),
            Event::AddCard { card_id, node_id } => {
                self.set_node_id(card_id.clone(), node_id.clone());
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use deskc_hir::expr::{Expr, Literal};
    use deskc_ids::FileId;
    use deskc_types::Type;
    use dkernel_card::patch::{AttributePatch, ChildrenPatch, ContentPatch};
    use uuid::Uuid;

    use super::*;

    #[test]
    fn add_node() {
        let mut db = Hirs::default();
        let node_id = NodeId(Uuid::new_v4());

        db.handle_event(&Event::AddNode {
            node_id: node_id.clone(),
            file_id: FileId(Uuid::new_v4()),
            content: Content::String("a".into()),
        });

        assert_eq!(db.content(node_id.clone()), Content::String("a".into()));
        assert_eq!(db.children(node_id.clone()), vec![]);
        assert_eq!(db.attributes(node_id), Attributes::default());
    }

    #[test]
    fn patch_content() {
        let mut db = Hirs::default();
        let node_id = handle_add_node(&mut db);

        db.handle_event(&Event::PatchContent {
            node_id: node_id.clone(),
            patch: ContentPatch::Replace(Content::String("b".into())),
        });

        assert_eq!(db.content(node_id), Content::String("b".into()));
    }

    #[test]
    fn patch_children() {
        let mut db = Hirs::default();
        let node_id = handle_add_node(&mut db);

        db.handle_event(&Event::PatchChildren {
            node_id: node_id.clone(),
            patch: ChildrenPatch::Insert {
                index: 0,
                node: NodeRef::Hole,
            },
        });

        assert_eq!(db.children(node_id), vec![NodeRef::Hole]);
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
            db.attributes(node_id),
            vec![(Type::Number, Expr::Literal(Literal::Integer(0)))]
                .into_iter()
                .collect()
        );
    }

    #[test]
    fn add_card() {
        let mut db = Hirs::default();
        let node_id = handle_add_node(&mut db);
        let card_id = CardId(Uuid::new_v4());

        db.handle_event(&Event::AddCard {
            card_id: card_id.clone(),
            node_id: node_id.clone(),
        });

        assert_eq!(db.node_id(card_id), node_id);
    }

    fn handle_add_node(db: &mut Hirs) -> NodeId {
        let node_id = NodeId(Uuid::new_v4());
        db.handle_event(&Event::AddNode {
            node_id: node_id.clone(),
            file_id: FileId(Uuid::new_v4()),
            content: Content::String("a".into()),
        });
        node_id
    }
}
