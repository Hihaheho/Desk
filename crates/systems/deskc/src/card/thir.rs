use deskc_ids::CardId;
use deskc_thir::TypedHir;

use super::{CardQueries, QueryResult};

pub(super) fn thir(db: &dyn CardQueries, id: CardId) -> QueryResult<TypedHir> {
    todo!()
}
