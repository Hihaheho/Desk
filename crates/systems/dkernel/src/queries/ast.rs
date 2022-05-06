use deskc_ast::{expr::Expr, span::Spanned};
use deskc_ids::CardId;

use super::{KernelQueries, KernelResult};

pub(super) fn ast(db: &dyn KernelQueries, id: CardId) -> KernelResult<Spanned<Expr>> {
    todo!()
}
