use deskc_hir::{expr::Expr, meta::WithMeta};
use deskc_ids::CardId;

use super::{KernelQueries, KernelResult};

pub(super) fn hir(db: &dyn KernelQueries, id: CardId) -> KernelResult<WithMeta<Expr>> {
    todo!()
}
