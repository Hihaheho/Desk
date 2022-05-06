use deskc_hir::{expr::Expr, meta::WithMeta};
use deskc_ids::CardId;

use crate::query_result::QueryResult;

use super::KernelQueries;

pub(super) fn hir(db: &dyn KernelQueries, id: CardId) -> QueryResult<WithMeta<Expr>> {
    todo!()
}
