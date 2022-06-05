use deskc_hir::{expr::Expr, meta::WithMeta};
use deskc_ids::CardId;

use crate::query_result::QueryResult;

use super::CardQueries;

pub(super) fn hir(_db: &dyn CardQueries, _id: CardId) -> QueryResult<WithMeta<Expr>> {
    todo!()
}
