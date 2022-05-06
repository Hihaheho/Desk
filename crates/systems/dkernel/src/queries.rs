mod amir;
mod ast;
mod build_ast;
mod execution_context;
mod hir;
mod mir;
mod thir;
mod typed_amir;

use std::sync::Arc;

use amir::amir;
use ast::ast;
use build_ast::build_ast;
use deskc_amir::amir::Amirs;
use deskc_ast::span::Spanned;
use deskc_hir::meta::WithMeta;
use deskc_ids::CardId;
use deskc_mir::{environment::Environment, mir::Mirs};
use deskc_thir::TypedHir;
use execution_context::execution_context;
use hir::hir;
use mir::mir;
use thir::thir;
// use typed_amir::typed_amir;

use dkernel_card::{
    content::Content,
    flat_node::{Attributes, NodeRef},
    node::NodeId,
};

#[salsa::query_group(KernelStorage)]
pub trait KernelQueries {
    #[salsa::input]
    fn content(&self, id: NodeId) -> Content;
    #[salsa::input]
    fn children(&self, id: NodeId) -> Vec<NodeRef>;
    #[salsa::input]
    fn attributes(&self, id: NodeId) -> Attributes;
    #[salsa::input]
    fn textual_card(&self, id: CardId) -> Option<Arc<String>>;
    fn build_ast(&self, id: NodeId) -> KernelResult<Spanned<deskc_ast::expr::Expr>>;
    fn ast(&self, id: CardId) -> KernelResult<Spanned<deskc_ast::expr::Expr>>;
    fn hir(&self, id: CardId) -> KernelResult<WithMeta<deskc_hir::expr::Expr>>;
    fn thir(&self, id: CardId) -> KernelResult<TypedHir>;
    fn amir(&self, id: CardId) -> KernelResult<Amirs>;
    fn mir(&self, id: CardId) -> KernelResult<Mirs>;
    fn execution_context(&self, id: CardId) -> KernelResult<Environment>;
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

impl<T> From<T> for KernelError
where
    T: std::error::Error + Send + Sync + 'static,
{
    fn from(error: T) -> Self {
        KernelError(Arc::new(Box::new(error)))
    }
}
