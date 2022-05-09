use deskc_ast::{expr::Expr, span::Spanned};
use deskc_ids::CardId;

use crate::query_result::QueryResult;

use super::Queries;

pub(super) fn ast(db: &dyn Queries, id: CardId) -> QueryResult<Spanned<Expr>> {
    todo!()
}
