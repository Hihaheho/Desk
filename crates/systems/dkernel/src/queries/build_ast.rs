use deskc_ast::{expr::Expr, span::Spanned};
use dkernel_card::node::NodeId;

use super::{KernelQueries, KernelResult};

pub(super) fn build_ast(db: &dyn KernelQueries, id: NodeId) -> KernelResult<Spanned<Expr>> {
    todo!()
}
