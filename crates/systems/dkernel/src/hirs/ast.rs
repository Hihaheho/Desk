use deskc_ast::{expr::Expr, span::Spanned};
use deskc_ids::CardId;

use crate::query_result::QueryResult;

use super::HirQueries;

pub(super) fn ast(_db: &dyn HirQueries, _id: CardId) -> QueryResult<Spanned<Expr>> {
    todo!()
}
