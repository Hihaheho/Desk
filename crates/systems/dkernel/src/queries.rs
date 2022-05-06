mod amir;
mod ast;
mod build_ast;
mod execution_context;
mod hir;
mod is_root;
mod mir;
mod thir;
mod typed_amir;

use std::sync::Arc;

use amir::amir;
use ast::ast;
use build_ast::build_ast;
use deskc_ast::span::Spanned;
use deskc_hir::meta::WithMeta;
use deskc_ids::CardId;
use hir::hir;
use is_root::is_root;
use mir::mir;
use thir::thir;
// use execution_context::execution_context;
// use textual_card::textual_card;
// use typed_amir::typed_amir;

use dkernel_card::{
    content::Content,
    flat_node::{Attributes, NodeRef},
    node::NodeId,
};

#[salsa::query_group(KernelStorage)]
pub trait KernelQueries {
    #[salsa::input]
    fn references(&self, id: NodeId) -> Vec<NodeId>;
    #[salsa::input]
    fn content(&self, id: NodeId) -> Content;
    #[salsa::input]
    fn children(&self, id: NodeId) -> Vec<NodeRef>;
    #[salsa::input]
    fn attributes(&self, id: NodeId) -> Attributes;
    #[salsa::input]
    fn textual_card(&self, id: CardId) -> Option<Arc<String>>;
    fn is_root(&self, id: NodeId) -> bool;
    fn build_ast(&self, id: NodeId) -> KernelResult<Spanned<deskc_ast::expr::Expr>>;
    fn ast(&self, id: CardId) -> KernelResult<Spanned<deskc_ast::expr::Expr>>;
    fn hir(&self, id: CardId) -> KernelResult<WithMeta<deskc_hir::expr::Expr>>;
    fn thir(&self, id: CardId) -> KernelResult<deskc_thir::TypedHir>;
    fn amir(&self, id: CardId) -> KernelResult<deskc_amir::amir::Amirs>;
    fn mir(&self, id: CardId) -> KernelResult<deskc_mir::mir::Mirs>;
}

pub type KernelResult<T> = Result<Arc<T>, KernelError>;

#[derive(Debug, Clone)]
pub struct KernelError(pub Arc<Box<dyn std::error::Error + Send + Sync + 'static>>);

impl PartialEq for KernelError {
    fn eq(&self, _other: &Self) -> bool {
        // always returns true to avoid recomputation on error
        true
    }
}
impl Eq for KernelError {}

impl From<Box<dyn std::error::Error + Send + Sync + 'static>> for KernelError {
    fn from(e: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        KernelError(Arc::new(e))
    }
}
