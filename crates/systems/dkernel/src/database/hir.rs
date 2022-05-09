use deskc_hir::{expr::Expr, meta::WithMeta};
use deskc_ids::CardId;

use crate::query_result::QueryResult;

use super::Queries;

pub(super) fn hir(db: &dyn Queries, id: CardId) -> QueryResult<WithMeta<Expr>> {
    todo!()
}
