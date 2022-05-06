use deskc_ast::{expr::Expr, span::Spanned};
use dkernel_card::{
    content::Content,
    node::{Node, NodeId},
};

use super::{KernelQueries, KernelResult};

pub(super) fn build_ast(db: &dyn KernelQueries, id: NodeId) -> KernelResult<Node> {
    if let Content::Source(source) = db.content(id.clone()) {
    } else {
    }
    todo!()
}
