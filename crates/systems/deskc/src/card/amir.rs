use deskc_amir::amir::Amirs;
use deskc_ids::CardId;

use super::{CardQueries, QueryResult};

pub(super) fn amir(_db: &dyn CardQueries, _id: CardId) -> QueryResult<Amirs> {
    todo!()
}
