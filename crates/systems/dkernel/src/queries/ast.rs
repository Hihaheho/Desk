use deskc_ast::{expr::Expr, span::Spanned};
use deskc_ids::CardId;

use crate::query_result::QueryResult;

use super::KernelQueries;

pub(super) fn ast(db: &dyn KernelQueries, id: CardId) -> QueryResult<Spanned<Expr>> {
    todo!()
}
