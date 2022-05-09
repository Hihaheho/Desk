mod ast;
mod build_ast;
mod flat_node;
mod hir;
mod typed_amir;

use std::sync::Arc;

use ast::ast;
use build_ast::build_ast;
use deskc_ast::span::Spanned;
use deskc_hir::meta::WithMeta;
use deskc_ids::CardId;
use flat_node::flat_node;
use hir::hir;

use dkernel_card::{
    content::Content,
    flat_node::{Attributes, FlatNode, NodeRef},
    node::{Node, NodeId},
};

use crate::{query_result::QueryResult, event::Event};

#[salsa::query_group(KernelStorage)]
pub trait Queries {
    #[salsa::input]
    fn content(&self, id: NodeId) -> Content;
    #[salsa::input]
    fn children(&self, id: NodeId) -> Vec<NodeRef>;
    #[salsa::input]
    fn attributes(&self, id: NodeId) -> Attributes;
    fn flat_node(&self, id: NodeId) -> Arc<FlatNode>;
    fn build_ast(&self, id: NodeId) -> QueryResult<Node>;
    fn ast(&self, id: CardId) -> QueryResult<Spanned<deskc_ast::expr::Expr>>;
    fn hir(&self, id: CardId) -> QueryResult<WithMeta<deskc_hir::expr::Expr>>;
}

#[salsa::database(KernelStorage)]
#[derive(Default)]
pub struct Database {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for Database {
}

impl Database {
    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::AddOwner { user_id } => todo!(),
            Event::RemoveOwner { user_id } => todo!(),
            Event::AddNode { node_id, content } => todo!(),
            Event::RemoveNode(_) => todo!(),
            Event::PatchContent { node_id, patch } => todo!(),
            Event::PatchChildren { node_id, patch } => todo!(),
            Event::PatchAttribute { node_id, patch } => todo!(),
            Event::AddSnapshot { index, snapshot } => todo!(),
            Event::AddFile(_) => todo!(),
            Event::DeleteFile(_) => todo!(),
            Event::AddCard { card_id, node_id } => todo!(),
            Event::RemoveCard(_) => todo!(),
        }
    }
}
