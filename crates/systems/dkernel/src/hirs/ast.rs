use deskc_ast::{expr::Expr, span::Spanned};
use deskc_ids::CardId;

use crate::query_result::QueryResult;

use super::HirQueries;

pub(super) fn ast(db: &dyn HirQueries, id: CardId) -> QueryResult<Spanned<Expr>> {
    todo!()
}
