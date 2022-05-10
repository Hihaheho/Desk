use deskc_ids::CardId;
use deskc_thir::TypedHir;

use super::{CardQueries, QueryResult};

pub(super) fn thir(_db: &dyn CardQueries, _id: CardId) -> QueryResult<TypedHir> {
    todo!()
}
