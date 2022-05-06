mod ast;
mod build_ast;
mod flat_node;
mod hir;
mod typed_amir;

use std::sync::Arc;

use ast::ast;
use build_ast::build_ast;
use deskc_ast::span::Spanned;
use deskc_ids::CardId;
use flat_node::flat_node;

use dkernel_card::{
    content::Content,
    flat_node::{Attributes, FlatNode, NodeRef},
    node::{Node, NodeId},
};

use crate::query_result::QueryResult;

#[salsa::query_group(KernelStorage)]
pub trait KernelQueries {
    #[salsa::input]
    fn content(&self, id: NodeId) -> Content;
    #[salsa::input]
    fn children(&self, id: NodeId) -> Vec<NodeRef>;
    #[salsa::input]
    fn attributes(&self, id: NodeId) -> Attributes;
    fn flat_node(&self, id: NodeId) -> Arc<FlatNode>;
    fn build_ast(&self, id: NodeId) -> QueryResult<Node>;
    fn ast(&self, id: CardId) -> QueryResult<Spanned<deskc_ast::expr::Expr>>;
}

#[salsa::database(KernelStorage)]
#[derive(Default)]
pub struct KernelDatabase {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for KernelDatabase {}
