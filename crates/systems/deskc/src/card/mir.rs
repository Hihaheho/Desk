use deskc_ids::CardId;
use deskc_mir::mir::Mirs;

use super::{CardQueries, QueryResult};

pub(super) fn mir(_db: &dyn CardQueries, _id: CardId) -> QueryResult<Mirs> {
    todo!()
}
