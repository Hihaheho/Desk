mod ast;
mod build_ast;
mod hir;
mod typed_amir;

use ast::ast;
use build_ast::build_ast;
use deskc_ast::span::Spanned;
use deskc_ids::CardId;

use dkernel_card::{
    content::Content,
    flat_node::{Attributes, NodeRef},
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
    // #[salsa::input]
    // fn definition(&self, id: CardId, uuid: Uuid) -> QueryResult<Amirs>;
    // #[salsa::input]
    // fn latest_definition(&self, id: CardId) -> Uuid;
    fn build_ast(&self, id: NodeId) -> QueryResult<Node>;
    fn ast(&self, id: CardId) -> QueryResult<Spanned<deskc_ast::expr::Expr>>;
}
